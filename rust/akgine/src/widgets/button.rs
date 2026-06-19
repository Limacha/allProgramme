use std::sync::Arc;

use eframe::egui;

pub struct Button {
    id: String,
    label: String,
    icon: Option<Arc<[u8]>>,
    btnSize: egui::Vec2,
    iconSize: egui::Vec2,
    textSize: f32,
    layoutDirection: egui::Direction, // TopDown, BottomUp, LeftToRight, RightToLeft
}
impl Button {
    pub fn new(
        id: String,
        label: String,
        icon: Option<Arc<[u8]>>,
        btnSize: egui::Vec2,
        iconSize: egui::Vec2,
        textSize: f32,
        layoutDirection: egui::Direction,
    ) -> Self {
        Self {
            id,
            label,
            icon,
            btnSize,
            iconSize,
            textSize,
            layoutDirection,
        }
    }

    pub fn ui(&self, ui: &mut egui::Ui) -> bool {
        // reserve lespace et obtient la premiere interaction
        let (rect, mut response) = ui.allocate_exact_size(self.btnSize, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            // dessine le fond avant le texte/image
            if response.hovered() || response.is_pointer_button_down_on() {
                ui.painter()
                    .rect_filled(rect, 4.0, egui::Color32::from_white_alpha(20));
            }

            let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(rect));

            let layout = egui::Layout::from_main_dir_and_cross_align(
                self.layoutDirection,
                egui::Align::Center,
            )
            .with_cross_justify(true);

            child_ui.with_layout(layout, |ui| {
                if let Some(iconBytes) = &self.icon {
                    let image: egui::ImageSource<'_> = egui::ImageSource::Bytes {
                        uri: std::borrow::Cow::Owned(self.id.clone()),
                        bytes: egui::load::Bytes::Shared(iconBytes.clone()),
                    };

                    ui.add(egui::Image::new(image).fit_to_exact_size(self.iconSize));
                }

                // rend le text non selectable
                ui.add(
                    egui::Label::new(egui::RichText::new(&self.label).size(self.textSize))
                        .selectable(false),
                );
            });
        }
        response = response.on_hover_cursor(egui::CursorIcon::PointingHand);

        response.clicked()
    }
}
