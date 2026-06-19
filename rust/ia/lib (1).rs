//! db_derive — procedural macro that auto-generates `impl DbRecord`.
//!
//! Usage
//! ─────
//! ```rust
//! #[derive(Clone, Debug, DbRecord)]
//! #[table_name("tasks")]            // optional — defaults to snake_case(Name)+"s"
//! pub struct Task {
//!     pub id:         i64,          // always skipped → becomes PK
//!     pub user_id:    i64,
//!     pub title:      String,
//!     #[column(default = "0")]
//!     pub done:       bool,
//!     #[column(default = "0")]
//!     pub deleted:    bool,
//!     #[column(default = "(unixepoch())")]
//!     pub updated_at: i64,
//! }
//! ```
//!
//! Per-field attribute  `#[column(...)]`
//! ─────────────────────────────────────
//! | key              | effect                                      |
//! |------------------|---------------------------------------------|
//! | `skip`           | exclude this field entirely                 |
//! | `nullable`       | omit `.not_null()` on a non-Option field    |
//! | `not_null`       | force `.not_null()` even on `Option<T>`     |
//! | `name = "col"`   | override the column name                   |
//! | `default = "v"`  | add `.default("v")`                        |

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Field, Fields, GenericArgument, PathArguments, Type,
};

// ─────────────────────────────────────────────────────────────────────────────
// Public entry-point
// ─────────────────────────────────────────────────────────────────────────────

#[proc_macro_derive(DbRecord, attributes(table_name, column))]
pub fn derive_db_record(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_db_record(&ast)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

// ─────────────────────────────────────────────────────────────────────────────
// Core code-generation
// ─────────────────────────────────────────────────────────────────────────────

fn impl_db_record(ast: &DeriveInput) -> syn::Result<TokenStream2> {
    let struct_name = &ast.ident;
    let table_name  = resolve_table_name(ast)?;

    // Only named-field structs are supported (enum/tuple structs are rejected).
    let named_fields = match &ast.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(f) => &f.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    struct_name,
                    "DbRecord requires a struct with named fields",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                struct_name,
                "DbRecord can only be derived for structs",
            ))
        }
    };

    // Build one `Column::new(...)...` expression per eligible field.
    let mut col_exprs: Vec<TokenStream2> = Vec::new();

    for field in named_fields {
        let ident    = field.ident.as_ref().unwrap();
        let name_str = ident.to_string();

        // ── Skip the primary key ────────────────────────────────────────────
        // A field named `id` is ALWAYS the PK; `columns()` must not list it.
        if name_str == "id" {
            continue;
        }

        let attrs = FieldAttrs::parse(field)?;

        // Honor explicit opt-out via `#[column(skip)]`.
        if attrs.skip {
            continue;
        }

        // ── Column name ────────────────────────────────────────────────────
        let col_name = attrs.name.as_deref().unwrap_or(&name_str);

        // ── Type mapping ───────────────────────────────────────────────────
        // Unwrap Option<T> to get the inner type for ColType resolution.
        let (is_option, inner) = unwrap_option(&field.ty);
        let col_type           = map_rust_type(inner.unwrap_or(&field.ty))?;

        // ── Build the expression ────────────────────────────────────────────
        let mut expr = quote! { Column::new(#col_name, #col_type) };

        // Nullability:
        //   • Option<T>          → nullable  (no .not_null())
        //   • #[column(nullable)] → nullable
        //   • everything else    → .not_null()
        // An explicit #[column(not_null)] overrides Option<T>.
        let nullable = (is_option || attrs.nullable) && !attrs.not_null;
        if !nullable {
            expr = quote! { #expr.not_null() };
        }

        if let Some(default) = &attrs.default {
            expr = quote! { #expr.default(#default) };
        }

        col_exprs.push(expr);
    }

    // Emit the full `impl DbRecord` block.
    Ok(quote! {
        impl DbRecord for #struct_name {
            fn table_name() -> &'static str {
                #table_name
            }

            fn columns() -> Vec<Column> {
                vec![ #(#col_exprs),* ]
            }
        }
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Per-field attribute parser
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Default)]
struct FieldAttrs {
    skip:     bool,
    nullable: bool,
    not_null: bool,           // explicit override for Option<T> fields
    name:     Option<String>,
    default:  Option<String>,
}

impl FieldAttrs {
    fn parse(field: &Field) -> syn::Result<Self> {
        let mut out = Self::default();

        for attr in &field.attrs {
            if !attr.path().is_ident("column") {
                continue;
            }

            // parse_nested_meta handles: #[column(key)] and #[column(key = "val")]
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("skip") {
                    out.skip = true;
                } else if meta.path.is_ident("nullable") {
                    out.nullable = true;
                } else if meta.path.is_ident("not_null") {
                    out.not_null = true;
                } else if meta.path.is_ident("name") {
                    let v = meta.value()?;
                    let s: syn::LitStr = v.parse()?;
                    out.name = Some(s.value());
                } else if meta.path.is_ident("default") {
                    let v = meta.value()?;
                    let s: syn::LitStr = v.parse()?;
                    out.default = Some(s.value());
                }
                Ok(())
            })?;
        }

        Ok(out)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Type helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Returns `(true, Some(&inner_type))` if `ty` is `Option<inner_type>`.
fn unwrap_option(ty: &Type) -> (bool, Option<&Type>) {
    if let Type::Path(tp) = ty {
        if let Some(seg) = tp.path.segments.last() {
            if seg.ident == "Option" {
                if let PathArguments::AngleBracketed(ab) = &seg.arguments {
                    if let Some(GenericArgument::Type(inner)) = ab.args.first() {
                        return (true, Some(inner));
                    }
                }
            }
        }
    }
    (false, None)
}

/// Maps a Rust primitive / std type to its `ColType` token.
///
/// Supported mappings
/// ──────────────────
/// i8 / i16 / i32 / i64 / u8 / u16 / u32 / u64 / isize / usize / bool  →  ColType::Integer
/// f32 / f64                                                               →  ColType::Real
/// String                                                                  →  ColType::Text
/// Vec<u8>                                                                 →  ColType::Blob
fn map_rust_type(ty: &Type) -> syn::Result<TokenStream2> {
    if let Type::Path(tp) = ty {
        if let Some(seg) = tp.path.segments.last() {
            let name = seg.ident.to_string();
            return match name.as_str() {
                "i8" | "i16" | "i32" | "i64"
                | "u8" | "u16" | "u32" | "u64"
                | "isize" | "usize" | "bool" => Ok(quote! { ColType::Integer }),

                "f32" | "f64" => Ok(quote! { ColType::Real }),

                "String" => Ok(quote! { ColType::Text }),

                // Vec<u8> is the only supported Vec variant (→ Blob).
                "Vec" => {
                    if let PathArguments::AngleBracketed(ab) = &seg.arguments {
                        if let Some(GenericArgument::Type(Type::Path(inner))) = ab.args.first() {
                            if inner.path.is_ident("u8") {
                                return Ok(quote! { ColType::Blob });
                            }
                        }
                    }
                    Err(syn::Error::new_spanned(
                        ty,
                        "Only Vec<u8> is supported as ColType::Blob",
                    ))
                }

                other => Err(syn::Error::new_spanned(
                    ty,
                    format!(
                        "Unsupported type `{other}`. \
                         Add #[column(skip)] or implement the mapping manually."
                    ),
                )),
            };
        }
    }
    Err(syn::Error::new_spanned(ty, "Cannot determine ColType for this type"))
}

// ─────────────────────────────────────────────────────────────────────────────
// Table-name resolution
// ─────────────────────────────────────────────────────────────────────────────

fn resolve_table_name(ast: &DeriveInput) -> syn::Result<String> {
    // Look for an explicit `#[table_name("tasks")]` attribute on the struct.
    for attr in &ast.attrs {
        if attr.path().is_ident("table_name") {
            let s: syn::LitStr = attr.parse_args()?;
            return Ok(s.value());
        }
    }
    // Default: PascalCase struct name → snake_case + "s"
    // e.g.  Task → tasks,  UserProfile → user_profiles
    Ok(pascal_to_snake(&ast.ident.to_string()) + "s")
}

/// Converts `PascalCase` to `snake_case`.
fn pascal_to_snake(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(ch.to_ascii_lowercase());
    }
    out
}
