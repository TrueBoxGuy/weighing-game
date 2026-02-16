#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::ui::*;

pub mod logic;
pub mod ui;
pub mod gamemodel;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AppMode {
    Game,
    Solver,
}

pub fn app() -> Element {
    let mode = use_signal(|| AppMode::Game);

    rsx! {
        div { class: "{APP_CONTAINER} relative",
            ModeSwitcher { mode: mode }

            match mode() {
                AppMode::Game => rsx! {
                    crate::ui::game::GameMode {}
                },
                AppMode::Solver => rsx! {
                    crate::ui::solver::SolverMode {}
                }
            }
        }
    }
}


#[component]
fn ModeSwitcher(mode: Signal<AppMode>) -> Element {
    rsx! {
        div { class: "{MODE_SWITCHER_CONTAINER}",
            button { 
                class: if mode() == AppMode::Game { MODE_BTN_ACTIVE } else { MODE_BTN_INACTIVE },
                onclick: move |_| mode.set(AppMode::Game),
                "Game Mode"
            }
            button { 
                class: if mode() == AppMode::Solver { MODE_BTN_ACTIVE } else { MODE_BTN_INACTIVE },
                onclick: move |_| mode.set(AppMode::Solver),
                "Solver Mode"
            }
        }
    }
}

