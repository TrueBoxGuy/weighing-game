use dioxus::prelude::*;
use crate::gamemodel::*;
use crate::ui::game::state::*;
use crate::ui::styles::*;

#[component]
pub fn ConfigPanel(logic: GameModeLogic) -> Element {
    let active_config = logic.state.read().config.clone();
    let mut pending_config = use_signal(|| active_config.clone());
    let mut dev_min_str = use_signal(|| active_config.deviation_min_factor.to_string());
    let mut dev_max_str = use_signal(|| active_config.deviation_max_factor.to_string());
    
    // Validation Logic
    let (is_bad_min_valid, is_bad_max_valid, is_dev_min_valid, is_dev_max_valid, is_valid) = {
        let c = pending_config.read();
        let bad_min = c.bad_coins.min <= c.bad_coins.max;
        let bad_max = c.bad_coins.max >= c.bad_coins.min && c.bad_coins.max <= c.num_coins;
        
        let dev_min = c.deviation_min_factor >= 0.1 && c.deviation_min_factor <= 10.0 
            && (c.deviation_min_factor - 1.0).abs() > 0.001
            && c.deviation_min_factor <= c.deviation_max_factor;
        let dev_max = c.deviation_max_factor >= 0.1 && c.deviation_max_factor <= 10.0 
            && (c.deviation_max_factor - 1.0).abs() > 0.001
            && c.deviation_max_factor >= c.deviation_min_factor;
        
        let global = c.num_coins >= 3 && c.num_coins <= 20
            && bad_min && bad_max
            && dev_min && dev_max;
            
        (bad_min, bad_max, dev_min, dev_max, global)
    };

    let is_changed = *pending_config.read() != active_config;

    // Setters
    let mut update_config = move |mut f: Box<dyn FnMut(&mut GameConfig)>| {
        let mut c = pending_config.read().clone();
        f(&mut c);
        pending_config.set(c);
    };

    rsx! {
        div { class: "{PANEL_BASE}",
            h3 { class: "{PANEL_HEADER}", "GAME CONFIG" }
            
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
                            is_valid: is_bad_min_valid,
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
                            is_valid: is_bad_max_valid,
                            on_change: move |val: String| {
                                if let Ok(n) = val.parse::<usize>() {
                                    update_config(Box::new(move |c| c.bad_coins.max = n));
                                }
                            }
                        }
                    }
                }

                // Deviation Control
                div {
                    label { 
                        class: "block text-sm font-medium text-neutral-300 mb-2", 
                        title: "Counterfeit weight (e.g. 0.8 for light, 1.2 for heavy). Range determines mystery.",
                        "Deviation Factor (Min - Max)" 
                    }
                    div { class: "flex gap-2 items-center",
                        ConfigInput {
                            value: dev_min_str(),
                            min: 0.1,
                            max: 10.0,
                            step: 0.1,
                            is_valid: is_dev_min_valid,
                            is_float: true,
                            on_change: move |val: String| {
                                dev_min_str.set(val.clone());
                                if let Ok(n) = val.parse::<f64>() {
                                    update_config(Box::new(move |c| c.deviation_min_factor = n));
                                }
                            }
                        }
                        span { class: "text-neutral-500", "-" }
                        ConfigInput {
                            value: dev_max_str(),
                            min: 0.1,
                            max: 10.0,
                            step: 0.1,
                            is_valid: is_dev_max_valid,
                            is_float: true,
                            on_change: move |val: String| {
                                dev_max_str.set(val.clone());
                                if let Ok(n) = val.parse::<f64>() {
                                    update_config(Box::new(move |c| c.deviation_max_factor = n));
                                }
                            }
                        }
                    }
                    div { class: "flex justify-between items-start mt-1",
                        p { class: "text-[10px] text-neutral-500", "e.g. 1.2 = 20% heavier" }
                        p { class: "text-[10px] text-neutral-500", "e.g. 0.8 = 20% lighter" }
                    }
                }
                
                button {
                    class: if is_valid && is_changed { "{BTN_RESTART_PENDING}" } 
                           else if !is_valid { "{BTN_RESTART} opacity-50 cursor-not-allowed" }
                           else { "{BTN_RESTART}" },
                    disabled: !is_valid,
                    onclick: move |_| {
                        if is_valid {
                            logic.state.set(GameState::new(pending_config.read().clone()));
                            logic.reset();
                        }
                    },
                    if !is_valid { "INVALID CONFIG" } else { "RESTART GAME" }
                }
            }
        }
    }
}

#[component]
fn ConfigInput(
    value: String,
    min: f64,
    max: f64,
    step: Option<f64>,
    is_valid: bool,
    is_float: Option<bool>,
    on_change: EventHandler<String>
) -> Element {
    let on_keydown = move |evt: KeyboardEvent| {
        let key = evt.key().to_string();
        let forbidden = if is_float.unwrap_or(false) {
            vec!["e", "E"]
        } else {
            vec!["e", "E", ".", "+", "-"] 
        };
        
        if forbidden.contains(&key.as_str()) {
            evt.prevent_default();
        }
    };

    rsx! {
        input {
            r#type: "number",
            min: "{min}",
            max: "{max}",
            step: "{step.unwrap_or(1.0)}",
            value: "{value}",
            class: if !is_valid { "{INPUT_NUMBER} {INPUT_INVALID}" } else { "{INPUT_NUMBER}" },
            onkeydown: on_keydown,
            oninput: move |e| on_change.call(e.value())
        }
    }
}

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
