use dioxus::prelude::*;
use crate::logic::*;
use crate::ui::game::state::*;
use crate::ui::styles::*;

#[component]
pub fn GuessingPanel(logic: GameModeLogic) -> Element {
    let config = logic.state.read().config.clone();
    let coins = 1..=config.num_coins;
    
    let mut accusations = use_signal(|| std::collections::HashMap::<usize, Weight>::new());
    
    let mut cycle_accusation = move |coin: usize| {
        let mut map = accusations.write();
        let current = *map.get(&coin).unwrap_or(&Weight::NORMAL);
        
        let next = if current.is_normal() {
            Weight::HEAVY
        } else if current.is_heavy() {
            Weight::LIGHT
        } else {
            Weight::NORMAL
        };
        
        if next.is_normal() {
            map.remove(&coin);
        } else {
            map.insert(coin, next);
        }
    };
    
    let submit_accusation = move |_| {
        let state = logic.state.read();
        let attempts = state.history.len();
        let map = accusations.read();
        
        let mut correct = true;
        let mut bad_coin_indices = Vec::new();
        
        for (i, w) in state.true_weights.iter().enumerate() {
            let coin_num = i + 1;
            if !w.is_normal() {
                bad_coin_indices.push(coin_num);
                let accusation = map.get(&coin_num).unwrap_or(&Weight::NORMAL);
                
                let matches = (w.is_heavy() && accusation.is_heavy()) ||
                              (w.is_light() && accusation.is_light());
                
                if !matches {
                    correct = false;
                    break;
                }
            } else {
                if let Some(acc) = map.get(&coin_num) {
                    if !acc.is_normal() {
                         correct = false;
                         break;
                    }
                }
            }
        }
        
        if correct {
             logic.status.set(GameStatus::Won(attempts));
        } else {
            logic.status.set(GameStatus::Lost(attempts));
        }
    };
    
    let get_coin_style = move |coin: usize| {
        let map = accusations.read();
        let w = map.get(&coin).unwrap_or(&Weight::NORMAL);
        
        if w.is_heavy() {
            "bg-red-900/50 text-red-200 border-2 border-red-500 shadow-[0_0_10px_rgba(239,68,68,0.4)]"
        } else if w.is_light() {
            "bg-blue-900/50 text-blue-200 border-2 border-blue-500 shadow-[0_0_10px_rgba(59,130,246,0.4)]"
        } else {
            "bg-neutral-800 text-neutral-400 border border-neutral-700 hover:border-neutral-500"
        }
    };

    rsx! {
        div { class: "{PANEL_BASE}",
             h3 { class: "{PANEL_HEADER}", "MAKE ACCUSATION" }
             p { class: "text-xs text-neutral-500 mb-4", "Tap on individual coins to cycle between heavy and light" }
             
             div { class: "grid grid-cols-4 sm:grid-cols-5 gap-2 mb-6",
                for coin in coins {
                    button {
                        class: "aspect-square rounded-lg font-bold text-sm transition-all {get_coin_style(coin)}",
                        onclick: move |_| cycle_accusation(coin),
                        div { "{coin}" }
                        div { class: "text-[9px] uppercase mt-1 px-1",
                            match accusations.read().get(&coin).unwrap_or(&Weight::NORMAL) {
                                w if w.is_heavy() => "Heavy",
                                w if w.is_light() => "Light",
                                _ => ""
                            }
                        }
                    }
                }
             }
             
             div { class: "space-y-2",
                 button {
                    class: "w-full py-3 bg-indigo-600 hover:bg-indigo-500 text-white font-bold rounded-xl shadow-lg transition-all",
                    onclick: submit_accusation,
                    "SUBMIT ACCUSATION"
                 }
                 
                 button {
                    class: "w-full py-2 bg-neutral-800 hover:bg-neutral-700 text-neutral-400 rounded-lg text-xs font-bold transition-all border border-neutral-700",
                    onclick: move |_| {
                        accusations.write().clear();
                    },
                    "CLEAR SELECTION"
                 }
             }
        }
    }
}
