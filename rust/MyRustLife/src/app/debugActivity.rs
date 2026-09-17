use crate::app::MainActivity;
use crate::app::stateManager::get_shared_state;
use crate::core::state;
use akgine::gui::context::UiContext;
use akgine::gui::navigation::activity::{Activity, ActivityContent, ActivityTrait};
use akgine::gui::widgets::Label;
use akgine::gui::widgets::Panel;

pub struct DebugActivity {
    pub activity: Activity,
}

impl DebugActivity {
    pub fn init() -> Self {
        let activity: Activity = Activity::new_with_activities(
            "DebugActivity",
            "Debug",
            include_bytes!("../../assets/icon/home_icon.png"),
            vec![Box::new(MainActivity::init())],
        );
        Self { activity }
    }
}

impl ActivityTrait for DebugActivity {
    fn activity(&self) -> &Activity {
        &self.activity
    }

    fn ui(&mut self, ctx: &mut UiContext) {
        // let context: egui::Context = ui.ctx().clone();

        // let state: std::sync::Arc<std::sync::Mutex<state::State>> =
        //     stateManager::get_shared_state(&context);

        let state: std::sync::Arc<std::sync::Mutex<state::State>> = get_shared_state(ctx);

        Panel::top("debugPanel").show(ctx, |inner_ctx| {
            // monitor size
            let (monitor_size, win_size) = inner_ctx.input(|i| {
                let vp = i.viewport();
                (vp.monitor_size, vp.inner_rect.map(|r| r.size()))
            });

            let screen_str: String = monitor_size
                .map(|m| format!("ecran:{}x{}", m.x as i32, m.y as i32))
                .unwrap_or_default();

            let win_str: String = win_size
                .map(|w| format!("fenetre:{}x{}", w.x as i32, w.y as i32))
                .unwrap_or_default();

            let (content_width, content_height) = inner_ctx.content_size();

            // delta time
            let deltaTime: f32 = inner_ctx.input(|inputState| inputState.stable_dt);
            let fps: f32 = if (deltaTime > 0.0) {
                1.0 / deltaTime
            } else {
                0.0
            };

            let stateLock: std::sync::MutexGuard<'_, state::State> = state.lock().unwrap();

            Label::text(
                "lblDebugPath",
                format!(
                    "path ({}): {}",
                    stateLock.router.index(),
                    stateLock.router.path()
                ),
            )
            .ui(inner_ctx);

            drop(stateLock);

            Label::text(
                "lblDebugContent",
                format!(
                    "content : {}x {} | {} | {}",
                    content_width as i32, content_height as i32, screen_str, win_str
                ),
            )
            .ui(inner_ctx);

            Label::text(
                "lblDebugInfo",
                format!("v0.0.1 | dt : {:.4}s | FPS : {:.1}", deltaTime, fps),
            )
            .ui(inner_ctx);

            let debugTexts: Vec<String> = state.lock().unwrap().debug_texts.clone();

            for (debugText) in debugTexts.iter() {
                Label::text("lblDebugText", debugText).ui(inner_ctx);
            }
        });

        match self.activity.content_mut() {
            ActivityContent::SubActivities { activities } => {
                activities.get_mut(0).unwrap().ui(ctx);
            }
            // ActivityContent::Pages { home, pages } => {

            // }
            _ => {
                // ui.label("no mainActivity set");
                Label::text("noMainAct", "no mainActivity set").ui(ctx);
            }
        }
    }
}
