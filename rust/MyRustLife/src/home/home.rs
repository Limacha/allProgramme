use akgine::navigation::page::{Page, PageTrait};
use eframe::egui;

pub struct Home {
    pub page: Page,
}

impl Home {
    pub fn init() -> Self {
        let page: Page = Page::new("Home", "Page principale", true, 1);
        Self { page }
    }
}

impl PageTrait for Home {
    fn page(&self) -> &Page {
        &self.page
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label(format!("{}", self.page.title()));
    }
}
