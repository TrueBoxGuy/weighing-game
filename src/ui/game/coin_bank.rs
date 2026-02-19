use dioxus::prelude::*;
use crate::ui::game::state::*;
use crate::ui::styles::*;

#[component]
pub fn CoinBank(logic: GameModeLogic, selection: Signal<Option<usize>>) -> Element {
    let total_coins = logic.state.read().config.num_coins;
    let current_weighing = logic.current_weighing.read();
    
    // Toggle for advanced mode (left/right click)
    let mut advanced_mode = use_signal(|| false);
    
    let available: Vec<usize> = (1..=total_coins)
        .filter(|c| !current_weighing.left.contains(c) && !current_weighing.right.contains(c))
        .collect();

    rsx! {
        div { class: "{PANEL_BASE}",
            div { class: "flex items-center justify-between mb-4",
                h3 { class: "{PANEL_HEADER} mb-0", "COIN BANK" }
                
                // Advanced Mode Toggle
                button {
                    class: format!(
                        "flex items-center gap-2 px-2 py-1 rounded text-xs font-bold transition-all {}",
                        if advanced_mode() { 
                            "bg-indigo-900/50 text-indigo-300 border border-indigo-500/50" 
                        } else { 
                            "text-neutral-500 hover:text-neutral-300" 
                        }
                    ),
                    onclick: move |_| advanced_mode.toggle(),
                    title: "Advanced Mode: use left and right click",
                    
                    // Mouse Icon
                    svg { 
                        class: "w-4 h-4", 
                        fill: "none", 
                        view_box: "0 0 24 24", 
                        stroke: "currentColor", 
                        stroke_width: "2",
                        path { d: "M15 3h-6a5 5 0 0 0-5 5v8a5 5 0 0 0 5 5h6a5 5 0 0 0 5-5v-8a5 5 0 0 0-5-5z" }
                        path { d: "M12 3v18" }
                        path { d: "M6 9h12" } 
                    }
                }
            }
            
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
                        // Left click handler
                        onclick: move |evt| {
                            if advanced_mode() {
                                // Add to Left Pan
                                let mut w = logic.current_weighing.write();
                                w.left.push(coin);
                                selection.set(None); // Clear selection if any
                            } else {
                                // Standard selection behavior
                                if selection() == Some(coin) {
                                    selection.set(None);
                                } else {
                                    selection.set(Some(coin));
                                }
                            }
                            evt.stop_propagation();
                        },
                        // Right click handler
                        oncontextmenu: move |evt| {
                            if advanced_mode() {
                                evt.prevent_default();
                                // Add to Right Pan
                                let mut w = logic.current_weighing.write();
                                w.right.push(coin);
                                selection.set(None);
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
