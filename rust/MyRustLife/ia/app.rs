// src/app/app.rs
//
// Top-level eframe App.
//
// Shows the full pattern:
//   - AppState has NO Repository<T> fields
//   - UI calls self.state.todo() / self.state.watchlist() every frame (cheap)
//   - FK usage: tasks shown grouped by category, add-task picks a category
//   - Mutations collected before being applied (Rust borrow rule)

use eframe::egui;

use crate::core::state::AppState;
use crate::watch_list::anime::AnimeStatus;
use crate::watch_list::scan::ScanStatus;
use crate::db;

// ── Tabs ─────────────────────────────────────────────────────────────────────

#[derive(PartialEq, Default)]
enum Tab { #[default] Todo, Anime, Scan }

// ── App ───────────────────────────────────────────────────────────────────────

pub struct App {
    state:        AppState,
    status:       String,
    active_tab:   Tab,

    // ── Todo inputs ───────────────────────────────────────────────────────────
    task_input:     String,
    cat_input:      String,
    selected_cat:   Option<i64>,   // category chosen for the next new task

    // ── WatchList inputs ──────────────────────────────────────────────────────
    anime_input:    String,
    scan_input:     String,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let state = db::open()
            .and_then(AppState::new)
            .expect("failed to open database");

        Self {
            state,
            status:       "Ready".into(),
            active_tab:   Tab::default(),
            task_input:   String::new(),
            cat_input:    String::new(),
            selected_cat: None,
            anime_input:  String::new(),
            scan_input:   String::new(),
        }
    }

    fn ok(&mut self, msg: impl Into<String>) { self.status = msg.into(); }
    fn err(&mut self, e: impl std::fmt::Display) { self.status = format!("❌ {e}"); }
}

// ── eframe::App ───────────────────────────────────────────────────────────────

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // ── Top nav ───────────────────────────────────────────────────────────
        egui::TopBottomPanel::top("nav").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let pending = self.state.todo().pending_count();
                ui.selectable_value(&mut self.active_tab, Tab::Todo,
                    format!("✅ Todo ({pending})"));
                ui.selectable_value(&mut self.active_tab, Tab::Anime, "🎬 Anime");
                ui.selectable_value(&mut self.active_tab, Tab::Scan,  "📖 Scan");
            });
        });

        // ── Status bar ────────────────────────────────────────────────────────
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.label(&self.status);
        });

        // ── Main panel ────────────────────────────────────────────────────────
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.active_tab {
                Tab::Todo  => self.ui_todo(ui),
                Tab::Anime => self.ui_anime(ui),
                Tab::Scan  => self.ui_scan(ui),
            }
        });
    }
}

// ── Todo screen ───────────────────────────────────────────────────────────────

impl App {
    fn ui_todo(&mut self, ui: &mut egui::Ui) {
        // ── Add category ──────────────────────────────────────────────────────
        ui.collapsing("➕ New category", |ui| {
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.cat_input)
                    .hint_text("Category name…").desired_width(220.0));
                if ui.button("Add").clicked() && !self.cat_input.trim().is_empty() {
                    match self.state.todo().add_category(&self.cat_input) {
                        Ok(id) => { self.ok(format!("Category #{id} added")); self.cat_input.clear(); }
                        Err(e) => self.err(e),
                    }
                }
            });
        });

        ui.separator();

        // ── Add task (with optional category picker) ──────────────────────────
        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(&mut self.task_input)
                .hint_text("New task…").desired_width(220.0));

            // Category picker dropdown
            let cats = self.state.todo().all_categories();
            let label = self.selected_cat
                .and_then(|id| cats.iter().find(|c| c.id == id))
                .map(|c| c.name.as_str())
                .unwrap_or("No category");

            egui::ComboBox::from_id_salt("cat_pick")
                .selected_text(label)
                .show_ui(ui, |ui| {
                    if ui.selectable_label(self.selected_cat.is_none(), "No category").clicked() {
                        self.selected_cat = None;
                    }
                    for cat in &cats {
                        if ui.selectable_label(self.selected_cat == Some(cat.id), &cat.name).clicked() {
                            self.selected_cat = Some(cat.id);
                        }
                    }
                });

            if ui.button("Add task").clicked() && !self.task_input.trim().is_empty() {
                let result = match self.selected_cat {
                    Some(cat_id) => self.state.todo().add_to_category(&self.task_input, cat_id),
                    None         => self.state.todo().add(&self.task_input),
                };
                match result {
                    Ok(id) => { self.ok(format!("Task #{id} added")); self.task_input.clear(); }
                    Err(e) => self.err(e),
                }
            }
        });

        ui.separator();

        // ── Tasks grouped by category ─────────────────────────────────────────
        // Collect mutations first (Rust borrow rule: can't borrow self.state
        // mutably while iterating over data that was borrowed from it)
        let grouped = self.state.todo().pending_by_category();
        let mut toggle_id:     Option<i64> = None;
        let mut delete_id:     Option<i64> = None;

        let mut group_names: Vec<String> = grouped.keys().cloned().collect();
        group_names.sort();

        egui::ScrollArea::vertical().id_salt("todo").show(ui, |ui| {
            for group in &group_names {
                let tasks = &grouped[group];
                ui.strong(group);
                for task in tasks {
                    ui.horizontal(|ui| {
                        if ui.button("☐").on_hover_text("Mark done").clicked() {
                            toggle_id = Some(task.id);
                        }
                        ui.label(&task.title);
                        if ui.small_button("🗑").clicked() { delete_id = Some(task.id); }
                    });
                }
                ui.add_space(4.0);
            }
        });

        // Apply mutations after the borrow ends
        if let Some(id) = toggle_id {
            if let Err(e) = self.state.todo().toggle(id) { self.err(e); }
            else { self.ok("Task updated"); }
        }
        if let Some(id) = delete_id {
            if let Err(e) = self.state.todo().remove(id) { self.err(e); }
            else { self.ok("Task removed"); }
        }

        // ── Completed tasks ───────────────────────────────────────────────────
        let completed = self.state.todo().completed();
        if !completed.is_empty() {
            ui.separator();
            ui.collapsing(format!("✅ Completed ({})", completed.len()), |ui| {
                let mut undo_id: Option<i64> = None;
                for task in &completed {
                    ui.horizontal(|ui| {
                        if ui.button("↩").on_hover_text("Undo").clicked() { undo_id = Some(task.id); }
                        ui.label(&task.title);
                    });
                }
                if let Some(id) = undo_id {
                    if let Err(e) = self.state.todo().uncomplete(id) { self.err(e); }
                }
            });
        }

        // ── Category list (side info) ─────────────────────────────────────────
        let cats = self.state.todo().all_categories();
        if !cats.is_empty() {
            ui.separator();
            ui.collapsing(format!("🏷 Categories ({})", cats.len()), |ui| {
                let mut del_cat: Option<i64> = None;
                for cat in &cats {
                    ui.horizontal(|ui| {
                        let count = self.state.todo().by_category(cat.id).len();
                        ui.label(format!("{} ({})", cat.name, count));
                        if ui.small_button("🗑").clicked() { del_cat = Some(cat.id); }
                    });
                }
                if let Some(id) = del_cat {
                    if let Err(e) = self.state.todo().remove_category(id) { self.err(e); }
                    else { self.ok("Category removed"); }
                }
            });
        }
    }
}

// ── Anime screen ──────────────────────────────────────────────────────────────

impl App {
    fn ui_anime(&mut self, ui: &mut egui::Ui) {
        ui.heading("Anime");
        ui.separator();

        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(&mut self.anime_input)
                .hint_text("Title…").desired_width(260.0));
            if ui.button("Add").clicked() && !self.anime_input.trim().is_empty() {
                match self.state.watchlist().add_anime(&self.anime_input) {
                    Ok(id) => { self.ok(format!("Anime #{id} added")); self.anime_input.clear(); }
                    Err(e) => self.err(e),
                }
            }
        });

        ui.separator();

        let list = self.state.watchlist().all_anime();
        let mut ep_inc:    Option<i64>                  = None;
        let mut set_status: Option<(i64, AnimeStatus)>  = None;
        let mut del_id:    Option<i64>                  = None;

        egui::ScrollArea::vertical().id_salt("anime").show(ui, |ui| {
            for a in &list {
                ui.horizontal(|ui| {
                    egui::ComboBox::new(format!("as{}", a.id), "")
                        .selected_text(a.status.label())
                        .show_ui(ui, |ui| {
                            for s in AnimeStatus::ALL {
                                if ui.selectable_label(&a.status == s, s.label()).clicked() {
                                    set_status = Some((a.id, s.clone()));
                                }
                            }
                        });
                    ui.label(&a.title);
                    ui.weak(format!("ep.{}", a.episode));
                    if ui.small_button("+1").clicked() { ep_inc = Some(a.id); }
                    if ui.small_button("🗑").clicked()  { del_id = Some(a.id); }
                });
            }
        });

        if let Some(id) = ep_inc { if let Err(e) = self.state.watchlist().next_episode(id) { self.err(e); } }
        if let Some((id, s)) = set_status { if let Err(e) = self.state.watchlist().set_anime_status(id, s) { self.err(e); } }
        if let Some(id) = del_id { if let Err(e) = self.state.watchlist().remove_anime(id) { self.err(e); } }
    }
}

// ── Scan screen ───────────────────────────────────────────────────────────────

impl App {
    fn ui_scan(&mut self, ui: &mut egui::Ui) {
        ui.heading("Scan / Manga");
        ui.separator();

        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(&mut self.scan_input)
                .hint_text("Title…").desired_width(260.0));
            if ui.button("Add").clicked() && !self.scan_input.trim().is_empty() {
                match self.state.watchlist().add_scan(&self.scan_input) {
                    Ok(id) => { self.ok(format!("Scan #{id} added")); self.scan_input.clear(); }
                    Err(e) => self.err(e),
                }
            }
        });

        ui.separator();

        let list = self.state.watchlist().all_scans();
        let mut ch_inc:     Option<i64>                 = None;
        let mut set_status: Option<(i64, ScanStatus)>   = None;
        let mut del_id:     Option<i64>                 = None;

        egui::ScrollArea::vertical().id_salt("scan").show(ui, |ui| {
            for s in &list {
                ui.horizontal(|ui| {
                    egui::ComboBox::new(format!("ss{}", s.id), "")
                        .selected_text(s.status.label())
                        .show_ui(ui, |ui| {
                            for st in ScanStatus::ALL {
                                if ui.selectable_label(&s.status == st, st.label()).clicked() {
                                    set_status = Some((s.id, st.clone()));
                                }
                            }
                        });
                    ui.label(&s.title);
                    ui.weak(format!("ch.{:.1}", s.chapter));
                    if ui.small_button("+1").clicked() { ch_inc = Some(s.id); }
                    if ui.small_button("🗑").clicked()  { del_id = Some(s.id); }
                });
            }
        });

        if let Some(id) = ch_inc { if let Err(e) = self.state.watchlist().advance_chapter(id, 1.0) { self.err(e); } }
        if let Some((id, st)) = set_status { if let Err(e) = self.state.watchlist().set_scan_status(id, st) { self.err(e); } }
        if let Some(id) = del_id { if let Err(e) = self.state.watchlist().remove_scan(id) { self.err(e); } }
    }
}
