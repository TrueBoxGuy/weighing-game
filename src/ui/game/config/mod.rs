use dioxus::prelude::*;
use crate::gamemodel::*;
use crate::ui::game::state::*;
use crate::ui::styles::*;
use super::config::inputs::ConfigInput;
use super::config::prompt::generate_ai_prompt;

#[component]
pub fn ConfigPanel(logic: GameModeLogic) -> Element {
    let active_config = logic.state.read().config.clone();
    let mut pending_config = use_signal(|| active_config.clone());
    let mut dev_min_str = use_signal(|| active_config.deviation_min_factor.to_string());
    let mut dev_max_str = use_signal(|| active_config.deviation_max_factor.to_string());
    let mut grouping_str = use_signal(|| active_config.grouping.map_or("".to_string(), |g| g.to_string()));

    // Validation Logic
    let validation = validation::validate_config(&pending_config.read());
    let is_changed = *pending_config.read() != active_config;

    // Setters
    let mut update_config = move |mut f: Box<dyn FnMut(&mut GameConfig)>| {
        let mut c = pending_config.read().clone();
        f(&mut c);
        pending_config.set(c);
    };

    rsx! {
        div { class: "{PANEL_BASE}",
            div { class: "flex justify-between items-center mb-4",
                h3 { class: "text-neutral-400 font-bold text-xs uppercase tracking-widest", "GAME CONFIG" }
                
                button {
                    class: "text-neutral-600 hover:text-indigo-400 transition-colors p-1 rounded hover:bg-neutral-800",
                    title: "Copy AI Prompt",
                    onclick: move |_| {
                        let prompt = generate_ai_prompt(&active_config);
                        spawn(async move {
                            if let Some(window) = web_sys::window() {
                                let navigator = window.navigator();
                                let clipboard = navigator.clipboard();
                                let _ = clipboard.write_text(&prompt);
                            }
                        });
                    },
                    span { class: "material-symbols-outlined text-sm select-none", "smart_toy" }
                }
            }
            
            div { class: "space-y-6",
                // Coins Control
                div {
                    div { class: "flex justify-between mb-2",
                        label { 
                            class: "text-sm font-medium text-neutral-300", 
                            title: "Number of coins in the puzzle (3-20)",
                            "Total Coins" 
                        }
                        span { class: "text-emerald-400 font-bold font-mono", "{pending_config.read().num_coins}" }
                    }
                    input { 
                        r#type: "range", min: "3", max: "20", 
                        value: "{pending_config.read().num_coins}",
                        class: "{INPUT_RANGE}",
                        oninput: move |e| {
                            if let Ok(n) = e.value().parse::<usize>() {
                                update_config(Box::new(move |c| {
                                    c.num_coins = n;
                                    if c.bad_coins.max > n { c.bad_coins.max = n; }
                                    if c.bad_coins.min > c.bad_coins.max { c.bad_coins.min = c.bad_coins.max; }
                                }));
                            }
                        }
                    }
                }

                // Bad Coins Control
                div {
                    label { 
                        class: "block text-sm font-medium text-neutral-300 mb-2", 
                        title: "Min/Max number of bad coins that can be generated",
                        "Bad Coin Count (Min - Max)" 
                    }
                    div { class: "flex gap-2 items-center",
                        ConfigInput {
                            value: pending_config.read().bad_coins.min.to_string(),
                            min: 0.0,
                            max: pending_config.read().num_coins as f64,
                            is_valid: validation.bad_min_valid,
                            on_change: move |val: String| {
                                if let Ok(n) = val.parse::<usize>() {
                                    update_config(Box::new(move |c| c.bad_coins.min = n));
                                }
                            }
                        }
                        span { class: "text-neutral-500", "-" }
                        ConfigInput {
                            value: pending_config.read().bad_coins.max.to_string(),
                            min: 0.0,
                            max: pending_config.read().num_coins as f64,
                            is_valid: validation.bad_max_valid,
                            on_change: move |val: String| {
                                if let Ok(n) = val.parse::<usize>() {
                                    update_config(Box::new(move |c| c.bad_coins.max = n));
                                }
                            }
                        }
                    }
                }

                // Deviation & Grouping Control
                // Deviation & Grouping Control
                div {
                    // Header Labels
                    div { class: "flex justify-between items-end mb-2",
                        label { 
                            class: "block text-sm font-medium text-neutral-300", 
                            title: "Counterfeit weight (e.g. 0.8 for light, 1.2 for heavy).",
                            "Deviation Factor (Min - Max)" 
                        }
                        label { 
                            class: "block text-sm font-medium text-neutral-300", 
                            title: "Grouping constraints.",
                            "Grouping" 
                        }
                    }

                    // Inputs Row
                    div { class: "flex justify-between items-center",
                        // Deviation Range
                        div { class: "flex gap-2 items-center",
                            ConfigInput {
                                value: dev_min_str(),
                                min: 0.1,
                                max: 10.0,
                                step: 0.1,
                                is_valid: validation.dev_min_valid,
                                is_float: true,
                                on_change: move |val: String| {
                                    dev_min_str.set(val.clone());
                                    if let Ok(n) = val.parse::<f64>() {
                                        update_config(Box::new(move |c| c.deviation_min_factor = n));
                                    }
                                }
                            }
                            span { class: "text-neutral-500 self-center", "-" }
                            ConfigInput {
                                value: dev_max_str(),
                                min: 0.1,
                                max: 10.0,
                                step: 0.1,
                                is_valid: validation.dev_max_valid,
                                is_float: true,
                                on_change: move |val: String| {
                                    dev_max_str.set(val.clone());
                                    if let Ok(n) = val.parse::<f64>() {
                                        update_config(Box::new(move |c| c.deviation_max_factor = n));
                                    }
                                }
                            }
                        }

                        // Grouping Control
                        div { 
                            class: "flex items-stretch bg-neutral-700 rounded border border-neutral-600 focus-within:ring-2 focus-within:ring-emerald-500 focus-within:border-emerald-500 overflow-hidden w-full max-w-[8rem]",
                            class: if !validation.grouping_valid { "border-red-500 focus-within:ring-red-500 focus-within:border-red-500 bg-red-900/20" } else { "" },
                            
                            // Number Input part
                            input {
                                r#type: "number",
                                min: "1",
                                max: "{pending_config.read().bad_coins.max}",
                                value: "{grouping_str}",
                                placeholder: "∞",
                                class: "w-full bg-transparent text-white text-center no-spinner outline-none px-2 py-1 font-mono",
                                onkeydown: move |evt: KeyboardEvent| {
                                    let key = evt.key().to_string();
                                    if vec!["e", "E", ".", "+", "-"].contains(&key.as_str()) {
                                        evt.prevent_default();
                                    }
                                },
                                oninput: move |e| {
                                    let val = e.value();
                                    grouping_str.set(val.clone());
                                    if let Ok(n) = val.parse::<usize>() {
                                        if n > 0 {
                                            update_config(Box::new(move |c| c.grouping = Some(n)));
                                        }
                                    } else if val.is_empty() {
                                        update_config(Box::new(move |c| c.grouping = None));
                                    }
                                }
                            }

                            // Separator
                            div { class: "w-[1px] bg-neutral-600" }

                            // Infinity Toggle Button
                            button {
                                class: format!("px-3 hover:bg-neutral-600 transition-colors flex items-center justify-center {}", 
                                    if pending_config.read().grouping.is_none() { 
                                        "text-indigo-400 bg-neutral-800 shadow-inner" 
                                    } else { 
                                        "text-neutral-400 hover:text-white" 
                                    }
                                ),
                                title: "Toggle Infinite Grouping (Unique weights)",
                                onclick: move |_| {
                                    update_config(Box::new(move |c| {
                                        if c.grouping.is_none() {
                                            c.grouping = Some(1);
                                            grouping_str.set("1".to_string());
                                        } else {
                                            c.grouping = None;
                                            grouping_str.set("".to_string());
                                        }
                                    }));
                                },
                                span { class: "material-symbols-outlined text-sm", "all_inclusive" }
                            }
                        }
                    }
                    div { class: "flex justify-between items-start mt-2",
                        div { 
                            p { class: "text-[10px] text-neutral-500", "e.g. 1.2 = 20% heavier" }
                            p { class: "text-[10px] text-neutral-500", "e.g. 0.8 = 20% lighter" }
                        }
                        div { class: "text-right",
                            p { class: "text-[10px] text-neutral-500", "(e.g. 1 means all bad" }
                            p { class: "text-[10px] text-neutral-500", "coins have same weight)" }
                        }
                    }
                }
                
                button {
                    class: if validation.is_valid && is_changed { "{BTN_RESTART_PENDING}" } 
                           else if !validation.is_valid { "{BTN_RESTART} opacity-50 cursor-not-allowed" }
                           else { "{BTN_RESTART}" },
                    disabled: !validation.is_valid,
                    onclick: move |_| {
                        if validation.is_valid {
                            logic.state.set(GameState::new(pending_config.read().clone()));
                            logic.reset();
                        }
                    },
                    if !validation.is_valid { "INVALID CONFIG" } else { "RESTART GAME" }
                }
            }
        }
    }
}

pub mod inputs;
pub mod prompt;
pub mod validation;

