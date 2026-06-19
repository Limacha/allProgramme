use akgine::navigation::activity::{Activity, ActivityTrait};

use crate::home::Home;

pub struct HomeActivity {
    activity: Activity,
}

impl HomeActivity {
    pub fn init() -> Self {
        let activity: Activity = Activity::new_with_pages(
            "HomeActivity",
            "Home",
            include_bytes!("../../assets/icon/home_icon.png"),
            vec![Box::new(Home::init())],
        );
        Self { activity }
    }
}

impl ActivityTrait for HomeActivity {
    fn activity(&self) -> &Activity {
        &self.activity
    }
}
