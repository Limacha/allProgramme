// use crate::core::state;
// use crate::{app::stateManager, todo::task::Task};
use crate::todo::Home;
use akgine::gui::context::UiContext;
use akgine::gui::navigation::activity::{Activity, ActivityContent, ActivityTrait};
use akgine::gui::widgets::Label;

pub struct TodoActivity {
    activity: Activity,
}

impl TodoActivity {
    pub fn init() -> Self {
        let activity: Activity = Activity::new_with_pages(
            "TodoActivity",
            "Todo",
            include_bytes!("../../assets/icon/todo_icon.png"),
            vec![Box::new(Home::init())],
        );
        Self { activity }
    }
}

impl ActivityTrait for TodoActivity {
    fn activity(&self) -> &Activity {
        &self.activity
    }

    fn ui(&mut self, ctx: &mut UiContext) {
        // ui.label(self.activity.title());

        // let context: &eframe::egui::Context = ui.ctx();
        // let state: std::sync::Arc<std::sync::Mutex<state::State>> =
        //     stateManager::get_shared_state(&context);

        // let mut repo;

        match self.activity.content_mut() {
            ActivityContent::Pages { pages } => {
                if let Some(home) = pages.iter_mut().next() {
                    home.ui(ctx);
                } else {
                    Label::text("lblNoPage", "pas de pages trouver.").ui(ctx);
                }
            }
            // ActivityContent::Pages { home, pages } => {

            // }
            _ => {}
        }
    }
}
