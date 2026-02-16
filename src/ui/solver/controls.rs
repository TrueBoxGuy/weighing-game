use dioxus::prelude::*;
use crate::logic::Weight;
use crate::ui::styles::*;

#[component]
pub fn ControlPanel(
    selected_coin: usize, 
    selected_weight: Weight,
    on_coin_change: EventHandler<usize>,
    on_weight_change: EventHandler<Weight>
) -> Element {
    rsx! {
        div { class: "{PANEL_CONTAINER}",
            div { class: "flex justify-center flex-wrap gap-1 md:gap-2 mb-6",
                for i in 1..=12 {
                    CoinButton { 
                        id: i, 
                        selected: selected_coin == i, 
                        on_click: move |_| on_coin_change.call(i)
                    }
                }
                CoinButton { 
                    id: 0, 
                    selected: selected_coin == 0, 
                    on_click: move |_| on_coin_change.call(0),
                    label: "None"
                }
            }

            if selected_coin != 0 {
                WeightSelector { 
                    selected_weight: selected_weight,
                    on_weight_change: on_weight_change
                }
            } else {
                div { class: "h-20" }
            }
        }
    }
}

#[component]
pub fn CoinButton(id: usize, selected: bool, on_click: EventHandler<MouseEvent>, label: Option<&'static str>) -> Element {
    let size_class = if label.is_some() { COIN_BTN_SIZE_LABEL } else { COIN_BTN_SIZE_DEFAULT };
    
    let color_class = if selected {
        if id == 0 { BTN_NONE_SELECTED } else { BTN_SELECTED }
    } else {
        if id == 0 { BTN_NONE_NORMAL } else { BTN_NORMAL }
    };

    rsx! {
        button {
            class: "{BTN_BASE} {size_class} {color_class}",
            onclick: move |e| on_click.call(e),
            "{label.unwrap_or(&id.to_string())}"
            if selected && id != 0 {
                span { 
                    class: "{COIN_BTN_BADGE}", 
                    "!" 
                }
            }
        }
    }
}

#[component]
pub fn WeightSelector(selected_weight: Weight, on_weight_change: EventHandler<Weight>) -> Element {
    let heavy_class = if selected_weight == Weight::HEAVY { BTN_WEIGHT_HEAVY } else { BTN_WEIGHT_HEAVY_INACTIVE };
    let light_class = if selected_weight == Weight::LIGHT { BTN_WEIGHT_LIGHT } else { BTN_WEIGHT_LIGHT_INACTIVE };

    rsx! {
        div { class: "h-20 flex justify-center items-center",
            div { class: "flex justify-center gap-6 animate-in fade-in slide-in-from-top-4 w-full max-w-md",
                button {
                    class: "{BTN_WEIGHT_BASE} {heavy_class}",
                    onclick: move |_| on_weight_change.call(Weight::HEAVY),
                    "HEAVIER"
                }
                button {
                    class: "{BTN_WEIGHT_BASE} {light_class}",
                    onclick: move |_| on_weight_change.call(Weight::LIGHT),
                    "LIGHTER"
                }
            }
        }
    }
}
