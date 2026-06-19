use akgine::navigation::activity::{Activity, ActivityTrait};

use crate::home::Home;

pub struct WatchListActivity {
    activity: Activity,
}

impl WatchListActivity {
    pub fn init() -> Self {
        let activity: Activity = Activity::new_with_pages(
            "WatchListActivity",
            "WatchList",
            include_bytes!("../../assets/icon/home_icon.png"),
            vec![Box::new(Home::init())],
        );
        Self { activity }
    }
}

impl ActivityTrait for WatchListActivity {
    fn activity(&self) -> &Activity {
        &self.activity
    }
}
