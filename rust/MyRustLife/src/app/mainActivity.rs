use std::sync::Arc;

use akgine::gui::context::UiContext;
use akgine::gui::navigation::activity::{Activity, ActivityContent, ActivityTrait};
use akgine::gui::types::{Align, Direction, Vec2};
use akgine::gui::widgets::Panel;
use akgine::gui::widgets::{Button, Label};

use crate::app::stateManager;
use crate::core::consts::*;
use crate::core::state;
use crate::home::HomeActivity;
use crate::todo::TodoActivity;
use crate::watchList::WatchListActivity;

pub struct MainActivity {
    pub activity: Activity,
}

impl MainActivity {
    pub fn init() -> Self {
        let activity: Activity = Activity::new_with_activities(
            "MainActivity",
            "Main",
            include_bytes!("../../assets/icon/home_icon.png"),
            vec![
                Box::new(HomeActivity::init()),
                Box::new(TodoActivity::init()),
                Box::new(WatchListActivity::init()),
            ],
        );
        Self { activity }
    }
}

impl ActivityTrait for MainActivity {
    fn activity(&self) -> &Activity {
        &self.activity
    }

    fn ui(&mut self, ctx: &mut UiContext) {
        // let context: egui::Context = ui.ctx().clone();

        let state: std::sync::Arc<std::sync::Mutex<state::State>> =
            stateManager::get_shared_state(ctx);

        // Menu du bas
        Panel::bottom("menuPagePanel")
            .inner_margin(PADDING as f32, BOTTOM_PADDING as f32)
            .show(ctx, |ctx| {
                // let boutons: [(&str, &[u8]); 4] = [
                //     ("Home", include_bytes!("../../assets/icon/home_icon.png")),
                //     ("Todo", include_bytes!("../../assets/icon/todo_icon.png")),
                //     ("Anim", include_bytes!("../../assets/icon/home_icon.png")),
                //     ("Scan", include_bytes!("../../assets/icon/home_icon.png")),
                // ];

                // let available: egui::Vec2 = ui.available_size(); // taille restante dans le panel
                // let btn_w: f32 = available.x / boutons.len() as f32; // largeur = espace / nb boutons
                // let btn_h: f32 = 56.0; // hauteur = tout le panel
                // let btn_size: egui::Vec2 = egui::vec2(btn_w, btn_h);

                // ui.columns(boutons.len(), |columns| {
                //     for (i, (id, bytes)) in boutons.iter().enumerate() {
                //         let button: Button = Button::new(
                //             id,
                //             id,
                //             bytes,
                //             btn_size,
                //             egui::vec2(30.0, 30.0),   // taille icône
                //             14.0,                     // taille texte
                //             egui::Direction::TopDown, // icône AU DESSUS du texte
                //         );

                //         if button.ui(&mut columns[i]) {
                //             // let index = state.lock().unwrap().router.index();
                //             // state.lock().unwrap().debug_texts.push(index.to_string());
                //             state.lock().unwrap().router.push(id);
                //         }
                //     }
                // });

                match self.activity.content() {
                    ActivityContent::SubActivities { activities } => {
                        let (availableX, _availableY) = ctx.content_size();
                        let btn_w: f32 = availableX / activities.len() as f32; // largeur = espace / nb boutons
                        let btn_h: f32 = 56.0; // hauteur = tout le panel
                        let btn_size: Vec2 = Vec2::new(btn_w, btn_h);

                        ctx.columns(activities.len(), |columns| {
                            for (i, act) in activities.iter().enumerate() {
                                // let activity = act.activity();
                                let id: String = act.activity().id().to_string();
                                let title: String = act.activity().title().to_string();
                                let icon: Arc<[u8]> = Arc::from(act.activity().icon());
                                let button: Button = Button::new(
                                    id.clone(),
                                    Some(title.clone()),
                                    Some(icon),
                                    Some(btn_size),
                                    Vec2::new(30.0, 30.0), // taille icône
                                    14.0,                  // taille texte
                                    Direction::TopDown,    // icône AU DESSUS du texte
                                    None,
                                    None,
                                    Align::Center,
                                );

                                if button.ui(&mut columns[i]) {
                                    // let index = state.lock().unwrap().router.index();
                                    // state.lock().unwrap().debug_texts.push(index.to_string());
                                    state.lock().unwrap().router.push(&id);
                                }
                            }
                        });
                    }
                    // ActivityContent::Pages { home, pages } => {

                    // }
                    _ => {
                        Label::text(
                            "lblNoActSet",
                            format!(
                                "no activities set in {}",
                                state.lock().unwrap().router.path()
                            ),
                        )
                        .ui(ctx);
                    }
                }
            });

        // match self.activity.content_mut() {
        //     ActivityContent::Pages { home, .. } => {
        //         home.ui(ui);
        //     }
        //     _ => {}
        // }
        /*
                let mut bottom_frame = egui::Frame::new();
                bottom_frame.inner_margin.bottom = BOTTOM_PADDING;
                bottom_frame.inner_margin.top = PADDING;

                egui::Panel::bottom("menuPagePanel")
                    .frame(bottom_frame)
                    .show_inside(ui, |ui| {
                        // let boutons: [(&str, &[u8]); 4] = [
                        //     ("Home", include_bytes!("../../assets/icon/home_icon.png")),
                        //     ("Todo", include_bytes!("../../assets/icon/todo_icon.png")),
                        //     ("Anim", include_bytes!("../../assets/icon/home_icon.png")),
                        //     ("Scan", include_bytes!("../../assets/icon/home_icon.png")),
                        // ];

                        // let available: egui::Vec2 = ui.available_size(); // taille restante dans le panel
                        // let btn_w: f32 = available.x / boutons.len() as f32; // largeur = espace / nb boutons
                        // let btn_h: f32 = 56.0; // hauteur = tout le panel
                        // let btn_size: egui::Vec2 = egui::vec2(btn_w, btn_h);

                        // ui.columns(boutons.len(), |columns| {
                        //     for (i, (id, bytes)) in boutons.iter().enumerate() {
                        //         let button: Button = Button::new(
                        //             id,
                        //             id,
                        //             bytes,
                        //             btn_size,
                        //             egui::vec2(30.0, 30.0),   // taille icône
                        //             14.0,                     // taille texte
                        //             egui::Direction::TopDown, // icône AU DESSUS du texte
                        //         );

                        //         if button.ui(&mut columns[i]) {
                        //             // let index = state.lock().unwrap().router.index();
                        //             // state.lock().unwrap().debug_texts.push(index.to_string());
                        //             state.lock().unwrap().router.push(id);
                        //         }
                        //     }
                        // });

                        match self.activity.content() {
                            ActivityContent::SubActivities { activities } => {
                                let available: egui::Vec2 = ui.available_size(); // taille restante dans le panel
                                let btn_w: f32 = available.x / activities.len() as f32; // largeur = espace / nb boutons
                                let btn_h: f32 = 56.0; // hauteur = tout le panel
                                let btn_size: Vec2 = Vec2::new(btn_w, btn_h);

                                ui.columns(activities.len(), |columns| {
                                    for (i, act) in activities.iter().enumerate() {
                                        // let activity = act.activity();
                                        let id: String = act.activity().id().to_string();
                                        let title: String = act.activity().title().to_string();
                                        let icon: Arc<[u8]> = Arc::from(act.activity().icon());
                                        let button: Button = Button::new(
                                            id.clone(),
                                            Some(title.clone()),
                                            Some(icon),
                                            Some(btn_size),
                                            Vec2::new(30.0, 30.0), // taille icône
                                            14.0,                  // taille texte
                                            Direction::TopDown,    // icône AU DESSUS du texte
                                            None,
                                            None,
                                            Align::Center,
                                        );

                                        if button.ui(&mut columns[i]) {
                                            // let index = state.lock().unwrap().router.index();
                                            // state.lock().unwrap().debug_texts.push(index.to_string());
                                            state.lock().unwrap().router.push(&id);
                                        }
                                    }
                                });
                            }
                            // ActivityContent::Pages { home, pages } => {

                            // }
                            _ => {
                                ui.label(format!(
                                    "no activities set in {}",
                                    state.lock().unwrap().router.path()
                                ));
                            }
                        }
                    });
        */

        Panel::central("mainCentralPanel").show(ctx, |ctx| {
            state.lock().unwrap().router.enter();
            match self.activity.content_mut() {
                ActivityContent::SubActivities { activities } => {
                    let currentTitle: String = state
                        .lock()
                        .unwrap()
                        .router
                        .current()
                        .unwrap_or("HomeActivity")
                        .to_string();

                    if let Some(activity) = activities
                        .iter_mut()
                        .find(|a| a.activity().id() == currentTitle)
                    {
                        activity.ui(ctx);
                    } else {
                        Label::text("lblPageNotFound", "pas de pages trouver.").ui(ctx);
                    }
                }
                // ActivityContent::Pages { home, pages } => {

                // }
                _ => {
                    Label::text(
                        "lblNoActSet",
                        format!(
                            "no activities set in {}",
                            state.lock().unwrap().router.path()
                        ),
                    )
                    .ui(ctx);
                }
            }
            state.lock().unwrap().router.exit();
        });
        ctx.request_repaint();
    }
}
