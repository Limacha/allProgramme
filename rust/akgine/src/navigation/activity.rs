use crate::navigation::page::PageTrait;
use eframe::egui;

pub enum ActivityContent {
    SubActivities {
        // mainActivity: Box<dyn ActivityTrait>,
        activities: Vec<Box<dyn ActivityTrait>>,
    },
    Pages {
        // home: Box<dyn PageTrait>,
        pages: Vec<Box<dyn PageTrait>>,
    },
}

pub struct Activity {
    id: &'static str,
    title: &'static str,
    icon: &'static [u8],
    content: ActivityContent,
}

impl Activity {
    fn new(
        id: &'static str,
        title: &'static str,
        icon: &'static [u8],
        content: ActivityContent,
    ) -> Self {
        Self {
            id,
            title,
            icon,
            content,
        }
    }

    pub fn new_with_activities(
        id: &'static str,
        title: &'static str,
        icon: &'static [u8],
        // mainActivity: Box<dyn ActivityTrait>,
        activities: Vec<Box<dyn ActivityTrait>>,
    ) -> Self {
        Self::new(
            id,
            title,
            icon,
            ActivityContent::SubActivities {
                // mainActivity,
                activities,
            },
        )
    }

    pub fn new_with_pages(
        id: &'static str,
        title: &'static str,
        icon: &'static [u8],
        // home: Box<dyn PageTrait>,
        pages: Vec<Box<dyn PageTrait>>,
    ) -> Self {
        Self::new(
            id,
            title,
            icon,
            ActivityContent::Pages {
                // home,
                pages,
            },
        )
    }

    pub fn id(&self) -> &str {
        self.id
    }

    pub fn title(&self) -> &str {
        self.title
    }

    pub fn icon(&self) -> &[u8] {
        self.icon
    }

    pub fn content(&self) -> &ActivityContent {
        &self.content
    }

    pub fn content_mut(&mut self) -> &mut ActivityContent {
        &mut self.content
    }
}

pub trait ActivityTrait {
    fn activity(&self) -> &Activity;

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label(self.activity().title());
    }
}
