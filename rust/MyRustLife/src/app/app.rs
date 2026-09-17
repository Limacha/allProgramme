use std::vec;

use akgine::gui::app::{AppTrait, CreationContext, install_image_loaders};
use akgine::gui::context::UiContext;
use akgine::gui::navigation::activity::ActivityTrait;
use akgine::gui::widgets::Label;

use crate::app::DebugActivity;
use crate::app::ReleaseActivity;
use crate::app::stateManager;
/*/
pub struct App {
    activities: Vec<Box<dyn ActivityTrait>>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // stateManager::init_shared_state(&cc.egui_ctx);
        egui_extras::install_image_loaders(&cc.egui_ctx);
        // stateManager::get_shared_state(&cc.egui_ctx)
        //     .lock()
        //     .unwrap()
        //     .debug_texts = vec![
        //     "try".to_string(),
        //     "this".to_string(),
        //     "working \n well".to_string(),
        // ];

        Self {
            activities: vec![
                // Box::new(MainActivity::init()),
                Box::new(ReleaseActivity::init()),
                Box::new(DebugActivity::init()),
            ],
        }
    }
}

// Implement the eframe::App trait so eframe knows how to draw our app.
impl eframe::App for App {
    // Called every frame by the eframe event loop.
    // `ui`    — the root UI region for this frame; panels are carved out of it.
    // `frame` — lets you control the native window (title, close, etc.) — unused here.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let context: egui::Context = ui.ctx().clone();
        let state: std::sync::Arc<std::sync::Mutex<crate::core::state::State>> =
            stateManager::get_shared_state(&context);

        let currentTitle = state
            .lock()
            .unwrap()
            .router
            .current()
            .unwrap_or("ReleaseActivity")
            .to_string();

        // self.mainActivity.ui(ui);

        if let Some(activity) = self
            .activities
            .iter_mut()
            .find(|a| a.activity().id() == currentTitle)
        {
            activity.ui(ui);
        } else {
            ui.label("pas de pages trouver.");
        }

        // context.request_repaint();
    }
}
*/

pub struct App {
    activities: Vec<Box<dyn ActivityTrait>>,
}

impl App {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        install_image_loaders(cc);

        Self {
            activities: vec![
                Box::new(ReleaseActivity::init()),
                Box::new(DebugActivity::init()),
            ],
        }
    }
}

impl AppTrait for App {
    fn ui(&mut self, ctx: &mut UiContext) {
        let state = stateManager::get_shared_state(ctx);

        let current_title = state
            .lock()
            .unwrap()
            .router
            .current()
            .unwrap_or("ReleaseActivity")
            .to_string();

        if let Some(activity) = self
            .activities
            .iter_mut()
            .find(|a| a.activity().id() == current_title)
        {
            activity.ui(ctx);
        } else {
            Label::text("lblNoPageFound", "pas de pages trouver.");
        }
    }
}
