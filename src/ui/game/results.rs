use dioxus::prelude::*;
use crate::ui::game::state::*;
use crate::ui::styles::*;

#[component]
pub fn ResultScreen(logic: GameModeLogic, won: bool, attempts: usize) -> Element {
    let title = if won { "VICTORY" } else { "DEFEAT" };
    let color = if won { "text-emerald-400" } else { "text-red-500" };
    let msg = if won { "You found all counterfeit coins!" } else { "Incorrect accusation." };
    
    // Calculate duration
    let duration = logic.state.read().start_time
        .map(|start| web_time::Instant::now().duration_since(start))
        .unwrap_or_default();
        
    let seconds = duration.as_secs();
    let time_str = format!("{:02}:{:02}", seconds / 60, seconds % 60);

    let state = logic.state.read();
    let bad_coins: Vec<(usize, &str)> = state.true_weights.iter().enumerate()
        .filter(|(_, w)| !w.is_normal())
        .map(|(i, w)| (i + 1, if w.is_heavy() { "Heavy" } else { "Light" }))
        .collect();

    rsx! {
        div { class: "{OVERLAY_CONTAINER}",
            div { class: "{OVERLAY_BOX}",
                h2 { class: "{RESULT_TITLE_BASE} {color}", "{title}" }
                p { class: "text-white text-lg mb-2", "{msg}" }
                
                div { class: "flex justify-center gap-8 mb-8 text-sm",
                    div {
                        p { class: "text-neutral-500 uppercase text-xs font-bold tracking-widest", "Weighings" }
                        p { class: "text-white font-mono text-xl", "{attempts}" }
                    }
                    div {
                        p { class: "text-neutral-500 uppercase text-xs font-bold tracking-widest", "Time" }
                        p { class: "text-white font-mono text-xl", "{time_str}" }
                    }
                }
                
                if !bad_coins.is_empty() {
                    div { class: "mb-8",
                        p { class: "text-neutral-500 text-xs uppercase tracking-widest mb-2", 
                            if bad_coins.len() > 1 { "Actual Bad Coins" } else { "Actual Bad Coin" }
                        }
                        div { class: "flex flex-wrap justify-center gap-2",
                            for (idx, kind) in bad_coins {
                                span { class: "px-3 py-1 bg-neutral-800 rounded-full text-xs border border-neutral-700",
                                    span { class: "text-white font-bold mr-1", "#{idx}" }
                                    span { class: if kind == "Heavy" { "text-red-400" } else { "text-blue-400" }, "({kind})" }
                                }
                            }
                        }
                    }
                } else {
                    p { class: "text-neutral-500 text-sm mb-8", "There were no bad coins." }
                }
                
                button {
                    class: "{OVERLAY_BTN}",
                    onclick: move |_| logic.reset(),
                    "PLAY AGAIN"
                }
            }
        }
    }
}
