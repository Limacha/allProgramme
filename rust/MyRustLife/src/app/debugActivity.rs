use crate::app::MainActivity;
use crate::app::stateManager;
use crate::core::consts::*;
use crate::core::state;
use akgine::navigation::activity::{Activity, ActivityContent, ActivityTrait};
use eframe::egui;

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

    fn ui(&mut self, ui: &mut egui::Ui) {
        let context: egui::Context = ui.ctx().clone();

        let state: std::sync::Arc<std::sync::Mutex<state::State>> =
            stateManager::get_shared_state(&context);

        let mut top_frame = egui::Frame::new();
        top_frame.inner_margin.top = TOP_PADDING;
        top_frame.inner_margin.bottom = PADDING;

        egui::Panel::top("debugPanel")
            .frame(top_frame)
            .show_inside(ui, |ui| {
                // monitor size
                let (monitor_size, win_size) = ui.input(|i| {
                    let vp = i.viewport();
                    (vp.monitor_size, vp.inner_rect.map(|r| r.size()))
                });

                let screen_str: String = monitor_size
                    .map(|m| format!("ecran:{}x{}", m.x as i32, m.y as i32))
                    .unwrap_or_default();

                let win_str: String = win_size
                    .map(|w| format!("fenetre:{}x{}", w.x as i32, w.y as i32))
                    .unwrap_or_default();

                let content: egui::Rect = context.content_rect();

                // delta time
                let deltaTime: f32 = ui.input(|inputState| inputState.stable_dt);
                let fps: f32 = if (deltaTime > 0.0) {
                    1.0 / deltaTime
                } else {
                    0.0
                };

                let stateLock: std::sync::MutexGuard<'_, state::State> = state.lock().unwrap();

                ui.label(format!(
                    "path ({}): {}",
                    stateLock.router.index(),
                    stateLock.router.path()
                ));

                drop(stateLock);

                ui.label(format!(
                    "content : {}x {} | {} | {}",
                    content.width() as i32,
                    content.height() as i32,
                    screen_str,
                    win_str
                ));

                ui.label(format!(
                    "v0.0.1 | dt : {:.4}s | FPS : {:.1}",
                    deltaTime, fps
                ));

                let debugTexts: Vec<String> = state.lock().unwrap().debug_texts.clone();

                for (debugText) in debugTexts.iter() {
                    ui.label(debugText);
                }
            });

        match self.activity.content_mut() {
            ActivityContent::SubActivities { activities } => {
                activities.get_mut(0).unwrap().ui(ui);
            }
            // ActivityContent::Pages { home, pages } => {

            // }
            _ => {
                ui.label("no mainActivity set");
            }
        }
    }
}
