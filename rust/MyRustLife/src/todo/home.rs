use crate::todo::repository::test;
use crate::{app::stateManager::get_shared_state, todo::repository::getUserTasks};
use akgine::database::DbError;
use akgine::navigation::page::{Page, PageTrait};
use akgine::widgets::Button;
use eframe::egui::{self, Color32, Direction, Vec2};

pub struct Home {
    pub page: Page,
}

impl Home {
    pub fn init() -> Self {
        let page: Page = Page::new("todoHome", "Page principale du todo", true, 1);
        Self { page }
    }
}

impl PageTrait for Home {
    fn page(&self) -> &Page {
        &self.page
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label(format!("putain: {}", self.page.title()));
        let state: std::sync::Arc<std::sync::Mutex<crate::core::state::State>> =
            get_shared_state(ui.ctx());

        let tButton: Button = Button::new(
            "tButton".to_string(),
            Some("bouton test".to_string()),
            None,
            None,
            Vec2::new(0.0, 0.0),
            16.0,
            Direction::TopDown,
            Some(Color32::from_rgb(150, 50, 150)),
            None,
            egui::Align::Min,
        );

        if (tButton.ui(ui)) {
            let tResult: Result<i64, DbError> = test(state.clone());

            state
                .lock()
                .unwrap()
                .debug_texts
                .push(format!("result : {:?}", tResult));

            let tasks: Result<Vec<super::Task>, DbError> = getUserTasks(state.clone());

            state
                .lock()
                .unwrap()
                .debug_texts
                .push(format!("{:?}", tasks));
        }
    }
}
