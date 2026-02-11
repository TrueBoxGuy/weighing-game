use dioxus::prelude::*;
use crate::logic::*;

use crate::ui::visuals::Scale;

#[component]
pub fn WeighingDisplay(path: Vec<(Weighing, Outcome)>, visible_steps: usize, selected_coin: usize, selected_weight: Weight) -> Element {
    rsx! {
        main { class: "max-w-6xl mx-auto grid grid-cols-1 lg:grid-cols-3 gap-8 mb-16 h-auto lg:min-h-[320px]",
            for (idx, (w, o)) in path.iter().enumerate() {
                if !w.left.is_empty() {
                    WeighingCard {
                        idx: idx,
                        weighing: w.clone(),
                        outcome: *o,
                        visible: idx < visible_steps,
                        fake_coin: selected_coin,
                        fake_weight: selected_weight
                    }
                }
            }
        }
    }
}

#[component]
pub fn WeighingCard(idx: usize, weighing: Weighing, outcome: Outcome, visible: bool, fake_coin: usize, fake_weight: Weight) -> Element {
    let anim_class = if visible { "opacity-100 scale-100" } else { "opacity-0 scale-95" };
    rsx! {
        div { class: "flex flex-col items-center transition-all duration-500 {anim_class}",
            h3 { class: "text-indigo-400 font-mono text-sm mb-4", "WEIGHING #{idx + 1}" }
            Scale { weighing: weighing, outcome: outcome, fake_coin: fake_coin, fake_weight: fake_weight, visible: visible }
            OutcomeLabel { outcome: outcome }
        }
    }
}

#[component]
pub fn OutcomeLabel(outcome: Outcome) -> Element {
    let (color, text) = match outcome {
        Outcome::Balanced => ("text-emerald-400", "Scales are Balanced"),
        Outcome::LeftHeavy => ("text-red-400", "Left Side is Heavier"),
        Outcome::RightHeavy => ("text-blue-400", "Right Side is Heavier"),
    };

    rsx! {
        div { class: "mt-4 text-center h-12",
            span { class: "text-neutral-500 text-xs block mb-1", "RESULT" }
            span { class: "text-sm font-black uppercase tracking-widest {color}", "{text}" }
        }
    }
}

#[component]
pub fn ConclusionDisplay(conclusion: String, visible: bool) -> Element {
    if !visible || conclusion.is_empty() {
        return rsx! { div { class: "h-32 mb-16" } };
    }

    let display = if conclusion == "None" {
        "No Forgery Found".to_string()
    } else if conclusion == "N/A" {
        "Impossible Case".to_string()
    } else {
        let coin = &conclusion[..conclusion.len()-1];
        let weight = if conclusion.ends_with('H') { "Heavier" } else { "Lighter" };
        format!("Coin {} is {}", coin, weight)
    };

    rsx! {
        div { class: "h-32 mb-16",
            div { class: "max-w-2xl mx-auto p-8 bg-[#1a1a1a] border border-indigo-500/30 rounded-3xl text-center shadow-2xl animate-in zoom-in-95 duration-700",
                span { class: "text-indigo-400 text-xs font-bold uppercase tracking-[0.3em] block mb-2", "Investigation Complete" }
                h2 { class: "text-3xl font-black text-white", "{display}" }
            }
        }
    }
}
