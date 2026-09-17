use crate::todo::repository::test;
use crate::{app::stateManager::get_shared_state, todo::repository::getUserTasks};

use akgine::database::DbError;
use akgine::gui::context::UiContext;
use akgine::gui::navigation::page::{Page, PageTrait};
use akgine::gui::types::{Align, Color, Direction, Vec2};
use akgine::gui::widgets::{Button, Label};

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

    fn ui(&mut self, ctx: &mut UiContext) {
        let labelTest: Label = Label::new(
            "tLabel",
            Some(format!("putain: {}", self.page.title())),
            None,
            None,
            Vec2::new(24.0, 24.0),
            16.0,
            Direction::LeftToRight,
            Some(Color::rgb(0, 0, 0)),
            Some(Color::rgb(255, 255, 255)),
            Align::Center,
            true,
        );
        // ui.label(format!("putain: {}", self.page.title()));
        let state: std::sync::Arc<std::sync::Mutex<crate::core::state::State>> =
            get_shared_state(ctx);

        let tButton: Button = Button::new(
            "tButton".to_string(),
            Some("bouton test".to_string()),
            None,
            None,
            Vec2::new(0.0, 0.0),
            16.0,
            Direction::TopDown,
            Some(Color::rgb(150, 50, 150)),
            None,
            Align::Min,
        );

        if (labelTest.ui(ctx)) {
            state
                .lock()
                .unwrap()
                .debug_texts
                .push(format!("labelTest :"));
        }

        if (tButton.ui(ctx)) {
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
