use dioxus::prelude::*;
use crate::ui::game::state::*;
use crate::ui::styles::*;

#[component]
pub fn CoinBank(logic: GameModeLogic, selection: Signal<Option<usize>>) -> Element {
    let total_coins = logic.state.read().config.num_coins;
    let current_weighing = logic.current_weighing.read();
    
    let available: Vec<usize> = (1..=total_coins)
        .filter(|c| !current_weighing.left.contains(c) && !current_weighing.right.contains(c))
        .collect();

    rsx! {
        div { class: "{PANEL_BASE}",
            h3 { class: "{PANEL_HEADER}", "COIN BANK" }
            div { class: "flex flex-wrap gap-2",
                for &coin in available.iter() {
                    button {
                        class: format!(
                            "{} {}",
                            COIN_BTN_BASE,
                            if Some(coin) == selection() { 
                                COIN_BTN_SELECTED
                            } else { 
                                COIN_BTN_NORMAL
                            }
                        ),
                        onclick: move |_| {
                            if selection() == Some(coin) {
                                selection.set(None);
                            } else {
                                selection.set(Some(coin));
                            }
                        },
                        "{coin}"
                    }
                }
            }
            if available.is_empty() {
                div { class: "text-neutral-600 italic text-sm text-center py-4", "All coins placed on scales" }
            }
        }
    }
}
