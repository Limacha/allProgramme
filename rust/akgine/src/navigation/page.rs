use eframe::egui;

pub struct Page {
    title: &'static str,
    description: &'static str,
    isShow: bool,
    pub ordre: u8,
}

impl Page {
    pub fn new(titre: &'static str, description: &'static str, isShow: bool, ordre: u8) -> Self {
        Self {
            title: titre,
            description: description,
            isShow,
            ordre,
        }
    }

    pub fn title(&self) -> &str {
        self.title
    }

    pub fn description(&self) -> &str {
        self.description
    }

    pub fn isShow(&self) -> bool {
        self.isShow
    }

    pub fn show(&mut self) {
        self.isShow = true;
    }

    pub fn hide(&mut self) {
        self.isShow = false;
    }
}

pub trait PageTrait {
    fn page(&self) -> &Page;

    fn ui(&mut self, ui: &mut egui::Ui);
}
