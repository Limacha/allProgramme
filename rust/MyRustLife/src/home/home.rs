use akgine::gui::context::UiContext;
use akgine::gui::navigation::page::{Page, PageTrait};
use akgine::gui::types::{Align, Color, Direction, Vec2};
use akgine::gui::widgets::Label;

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

    fn ui(&mut self, ctx: &mut UiContext) {
        Label::new(
            "lHomeHome",
            Some(format!("{}", self.page.title())),
            None,
            None,
            Vec2::new(0., 0.),
            16.,
            Direction::LeftToRight,
            Some(Color::rgba(0, 0, 0, 0)),
            Some(Color::rgb(255, 255, 255)),
            Align::Min,
            false,
        )
        .ui(ctx);
    }
}
