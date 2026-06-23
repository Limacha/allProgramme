use eframe::egui;
use std::sync::Arc;

pub struct Button {
    id: String,
    label: Option<String>,
    icon: Option<Arc<[u8]>>,
    /** 1) La taille est maintenant une option. Si None, le bouton s'ajuste au contenu. */
    btnSize: Option<egui::Vec2>,
    iconSize: egui::Vec2,
    textSize: f32,
    /** TopDown, BottomUp, LeftToRight, RightToLeft */
    layoutDirection: egui::Direction,
    /** Option pour la couleur de fond */
    bgColor: Option<egui::Color32>,
    /** Option pour la couleur du texte */
    textColor: Option<egui::Color32>,
    /** Alignement : Min (Gauche), Center (Milieu), Max (Droite) */
    alignment: egui::Align,
}

impl Button {
    pub fn new(
        id: String,
        label: Option<String>,
        icon: Option<Arc<[u8]>>,
        btnSize: Option<egui::Vec2>, /* 1) Ajout de l'Option ici */
        iconSize: egui::Vec2,
        textSize: f32,
        layoutDirection: egui::Direction,
        bgColor: Option<egui::Color32>,
        textColor: Option<egui::Color32>,
        alignment: egui::Align,
    ) -> Self {
        Self {
            id,
            label,
            icon,
            btnSize,
            iconSize,
            textSize,
            layoutDirection,
            bgColor,
            textColor,
            alignment,
        }
    }

    pub fn ui(&self, ui: &mut egui::Ui) -> bool {
        /* 2) Astuce du calque de fond (Shape::Noop) :
        Puisqu'on ne connaît pas toujours la taille à l'avance, on réserve
        un index dans le peintre (painter) AVANT de dessiner le texte/icône.
        On y place une forme vide temporaire. */
        let bg_idx = ui.painter().add(egui::Shape::Noop);

        /* Gestion dynamique de l'alignement selon la direction du layout */
        let mut layout: egui::Layout = egui::Layout::from_main_dir_and_cross_align(
            self.layoutDirection,
            if self.layoutDirection.is_vertical() {
                self.alignment
            } else {
                egui::Align::Center
            },
        );

        if self.layoutDirection.is_horizontal() {
            layout = layout.with_main_align(self.alignment);
        }

        /* 3) Création du conteneur adaptatif :
        Si `btnSize` est None, on utilise Vec2::ZERO. Cela indique à egui
        de réduire la zone (shrink-to-fit) pour qu'elle épouse parfaitement le contenu. */
        let desired_size = self.btnSize.unwrap_or(egui::Vec2::ZERO);

        let inner_res = ui.allocate_ui_with_layout(desired_size, layout, |ui| {
            /* 4) Si une taille fixe est demandée, on force les contraintes minimales
            et maximales de cette sous-interface. */
            if let Some(size) = self.btnSize {
                ui.set_min_size(size);
                ui.set_max_size(size);
            }

            if let Some(iconBytes) = &self.icon {
                let image: egui::ImageSource<'_> = egui::ImageSource::Bytes {
                    uri: std::borrow::Cow::Owned(self.id.clone()),
                    bytes: egui::load::Bytes::Shared(iconBytes.clone()),
                };

                ui.add(egui::Image::new(image).fit_to_exact_size(self.iconSize));
            }

            /* Affichage conditionnel du label */
            if let Some(label_text) = &self.label {
                let mut rich_text: egui::RichText =
                    egui::RichText::new(label_text).size(self.textSize);

                /* Application de la couleur de texte personnalisée */
                if let Some(txt_color) = self.textColor {
                    rich_text = rich_text.color(txt_color);
                }

                /* Rend le texte non selectionnable */
                ui.add(egui::Label::new(rich_text).selectable(false));
            }
        });

        /* 5) Récupération du rectangle final calculé par egui après avoir dessiné le contenu */
        let rect: egui::Rect = inner_res.response.rect;

        /* 6) Ajout de l'interaction (clic) manuellement sur ce rectangle dynamique */
        let mut response: egui::Response =
            ui.interact(rect, ui.id().with(&self.id), egui::Sense::click());

        if ui.is_rect_visible(rect) {
            /* 7) Mise à jour du calque : maintenant qu'on a les dimensions exactes (rect),
            on remplace la forme temporaire (Noop) par le rectangle coloré final.
            Il apparaîtra magiquement EN DESSOUS du texte et de l'icône ! */
            if let Some(bg) = self.bgColor {
                ui.painter()
                    .set(bg_idx, egui::Shape::rect_filled(rect, 4.0, bg));
            }

            /* Dessine l'effet visuel de survol ou de clic par-dessus le fond */
            if response.is_pointer_button_down_on() {
                ui.painter()
                    .rect_filled(rect, 4.0, egui::Color32::from_white_alpha(40));
            } else if response.hovered() {
                ui.painter()
                    .rect_filled(rect, 4.0, egui::Color32::from_white_alpha(20));
            }
        }

        response = response.on_hover_cursor(egui::CursorIcon::PointingHand);

        response.clicked()
    }
}
