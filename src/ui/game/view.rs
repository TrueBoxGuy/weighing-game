use dioxus::prelude::*;
use crate::ui::game::state::*;
use crate::ui::styles::*;
use crate::ui::game::config::*;
use crate::ui::game::coin_bank::*;
use crate::ui::game::scale::*;
use crate::ui::game::history::*;
use crate::ui::game::guessing::*;
use crate::ui::game::results::*;

#[component]
pub fn GameMode() -> Element {
    let logic = use_game_mode();

    rsx! {
        div { class: "{APP_CONTAINER} relative min-h-screen pb-20",
            div { class: "{HEADER_CONTAINER}",
                h1 { class: "{HEADER_H1}", "FIND THE FAKE" }
                p { class: "{HEADER_SUBTITLE}", "Use the scales to identify the counterfeit coin" }
            }

            match *logic.status.read() {
                GameStatus::Playing => rsx! {
                    PlayScreen { logic: logic }
                },
                GameStatus::Won(attempts) => rsx! {
                    ResultScreen { logic: logic, won: true, attempts: attempts }
                },
                GameStatus::Lost(attempts) => rsx! {
                    ResultScreen { logic: logic, won: false, attempts: attempts }
                }
            }
        }
    }
}

#[component]
fn PlayScreen(logic: GameModeLogic) -> Element {
    let coin_selection = use_signal(|| Option::<usize>::None);

    rsx! {
        div { class: "grid grid-cols-1 lg:grid-cols-3 gap-8",
            
            div { class: "space-y-6",
                ConfigPanel { logic: logic }
                CoinBank { logic: logic, selection: coin_selection }
            }

            div { class: "flex flex-col items-center justify-start",
                InteractiveScale { logic: logic, selection: coin_selection }
            }

            div { class: "space-y-6",
                GuessingPanel { logic: logic }
                HistoryLog { history: logic.state.read().history.clone() }
            }
        }
    }
}
