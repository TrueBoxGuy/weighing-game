use dioxus::prelude::*;
use crate::logic::*;
use crate::ui::game::state::*;
use crate::ui::styles::*;
use crate::ui::common::Scale;

#[component]
pub fn InteractiveScale(logic: GameModeLogic, selection: Signal<Option<usize>>) -> Element {
    let weighing = logic.current_weighing.read().clone();
    
    let on_drop_left = move |_| {
        if let Some(coin) = selection() {
            let mut w = logic.current_weighing.read().clone();
            if let Some(pos) = w.right.iter().position(|&c| c == coin) { w.right.remove(pos); }
            if !w.left.contains(&coin) { w.left.push(coin); w.left.sort(); }
            logic.current_weighing.set(w);
            selection.set(None);
        }
    };

    let on_drop_right = move |_| {
        if let Some(coin) = selection() {
            let mut w = logic.current_weighing.read().clone();
            if let Some(pos) = w.left.iter().position(|&c| c == coin) { w.left.remove(pos); }
            if !w.right.contains(&coin) { w.right.push(coin); w.right.sort(); }
            logic.current_weighing.set(w);
            selection.set(None);
        }
    };

    let on_weigh = move |_| {
        let w = logic.current_weighing.read().clone();
        if !w.left.is_empty() && !w.right.is_empty() {
             logic.state.write().weigh(w);
        }
    };

    let on_clear = move |_| {
        logic.current_weighing.set(Weighing { left: vec![], right: vec![] });
    };

    let mut remove_coin = move |coin: usize, is_left: bool| {
        let mut w = logic.current_weighing.read().clone();
        if is_left {
            if let Some(pos) = w.left.iter().position(|&c| c == coin) { w.left.remove(pos); }
        } else {
            if let Some(pos) = w.right.iter().position(|&c| c == coin) { w.right.remove(pos); }
        }
        logic.current_weighing.set(w);
    };
    
    let history = &logic.state.read().history;
    let last_outcome = if let Some((last_w, outcome)) = history.last() {
        if *last_w == weighing {
            *outcome
        } else {
            Outcome::Balanced 
        }
    } else {
        Outcome::Balanced
    };

    rsx! {
        div { class: "w-full flex flex-col items-center space-y-4",
            div { class: "relative group",
                Scale { 
                    weighing: weighing.clone(), 
                    outcome: last_outcome, 
                    fake_coin: 0, 
                    fake_weight: Weight::NORMAL, 
                    visible: true,
                    show_truth: false 
                }
                
                div { 
                    class: "{DROP_ZONE_LEFT}",
                    onclick: on_drop_left,
                    if selection().is_some() {
                        div { class: "{DROP_ZONE_ACTIVE}" }
                    }
                }

                div { 
                    class: "{DROP_ZONE_RIGHT}",
                    onclick: on_drop_right,
                    if selection().is_some() {
                         div { class: "{DROP_ZONE_ACTIVE}" }
                    }
                }
            }
            
            div { class: "flex w-full justify-between items-start px-4 text-xs font-mono text-neutral-500 gap-8",
                // Left Pan
                div { class: "flex gap-2 min-w-0 flex-1",
                    span { class: "shrink-0 font-bold text-neutral-400", "Left Pan:" }
                    div { class: "flex flex-wrap gap-x-2 gap-y-1",
                        for &c in weighing.left.iter() {
                            span { class: "cursor-pointer hover:text-red-400 underline", onclick: move |_| remove_coin(c, true), "{c}" }
                        }
                    }
                }
                // Right Pan
                div { class: "flex flex-row-reverse gap-2 min-w-0 flex-1",
                    span { class: "shrink-0 font-bold text-neutral-400", ": Right Pan" }
                    div { class: "flex flex-row-reverse flex-wrap gap-x-2 gap-y-1",
                        for &c in weighing.right.iter().rev() {
                            span { class: "cursor-pointer hover:text-red-400 underline", onclick: move |_| remove_coin(c, false), "{c}" }
                        }
                    }
                }
            }
            
            div { class: "flex gap-4",
                 button {
                    class: if last_outcome != Outcome::Balanced { "{BTN_CLEAR_PRIMARY}" } else { "{BTN_CLEAR_NORMAL}" },
                    onclick: on_clear,
                    "CLEAR"
                }
                button {
                    class: if last_outcome != Outcome::Balanced { "{WEIGH_BTN_DISABLED}" } else { "{WEIGH_BTN}" },
                    disabled: weighing.left.is_empty() || weighing.right.is_empty() || last_outcome != Outcome::Balanced,
                    onclick: on_weigh,
                    "WEIGH"
                }
            }
        }
    }
}
