// use eframe::egui;

use crate::app::MainActivity;
use crate::core::consts::*;

use akgine::gui::context::UiContext;
use akgine::gui::navigation::activity::{Activity, ActivityContent, ActivityTrait};
use akgine::gui::widgets::{Label, Panel};

pub struct ReleaseActivity {
    pub activity: Activity,
}

impl ReleaseActivity {
    pub fn init() -> Self {
        let activity: Activity = Activity::new_with_activities(
            "ReleaseActivity",
            "Release",
            include_bytes!("../../assets/icon/home_icon.png"),
            vec![Box::new(MainActivity::init())],
        );
        Self { activity }
    }
}

impl ActivityTrait for ReleaseActivity {
    fn activity(&self) -> &Activity {
        &self.activity
    }

    fn ui(&mut self, ctx: &mut UiContext) {
        if (TOP_PADDING > 0) {
            Panel::top("spacePanel")
                .inner_margin(TOP_PADDING as f32, PADDING as f32)
                .show(ctx, |_inner_ctx| {});
        }
        match self.activity.content_mut() {
            ActivityContent::SubActivities {
                // mainActivity,
                activities,
            } => {
                activities.get_mut(0).unwrap().ui(ctx);
            }
            // ActivityContent::Pages { home, pages } => {

            // }
            _ => {
                Label::text("noMainAct", "no mainActivity set").ui(ctx);
            }
        }
    }
}
