use eframe::egui;

use crate::app::MainActivity;
use crate::core::consts::*;
use akgine::navigation::activity::{Activity, ActivityContent, ActivityTrait};
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

    fn ui(&mut self, ui: &mut egui::Ui) {
        let mut top_frame = egui::Frame::new();
        top_frame.inner_margin.top = TOP_PADDING;
        top_frame.inner_margin.bottom = PADDING;

        #[allow(unused_variables)]
        egui::Panel::top("spacePanel")
            .frame(top_frame)
            .show_inside(ui, |ui| {});

        match self.activity.content_mut() {
            ActivityContent::SubActivities {
                // mainActivity,
                activities,
            } => {
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
