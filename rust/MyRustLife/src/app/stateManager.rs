use crate::core::state::{SharedStateRef, State};
use eframe::egui;
use std::sync::{Arc, Mutex};

fn state_id() -> egui::Id {
    egui::Id::new("appSharedState")
}

// /// Initialise l'état uniquement s'il n'existe pas déjà (Idempotent).
// pub fn init_shared_state(ctx: &egui::Context) -> SharedStateRef {
//     ctx.data_mut(|d| {
//         if let Some(existing_state) = d.get_temp::<SharedStateRef>(state_id()) {
//             // Déjà initialisé : on évite d'écraser l'état et on retourne la référence
//             existing_state
//         } else {
//             // Première fois : on initialise
//             let new_state: SharedStateRef = Arc::new(Mutex::new(State::default()));
//             d.insert_temp(state_id(), new_state.clone());
//             new_state
//         }
//     })
// }

// /// Récupère l'état de manière sûre, en retournant une Option pour obliger
// /// le développeur à gérer le cas où il a oublié d'appeler init().
// pub fn get_shared_state(ctx: &egui::Context) -> Option<SharedStateRef> {
//     ctx.data_mut(|d| d.get_temp::<SharedStateRef>(state_id()))
// }

/// Récupère l'état partagé, ou l'initialise s'il n'existe pas encore.
pub fn get_shared_state(ctx: &egui::Context) -> SharedStateRef {
    ctx.data_mut(|d| {
        // On essaie de récupérer l'état existant
        match d.get_temp::<SharedStateRef>(state_id()) {
            Some(state) => state, // Il existe, on le retourne
            None => {
                // Il n'existe pas encore, on l'initialise et on l'insère
                let new_state: SharedStateRef = Arc::new(Mutex::new(State::default()));
                d.insert_temp(state_id(), new_state.clone());
                new_state
            }
        }
    })
}
