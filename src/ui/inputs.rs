use dioxus::prelude::*;
use crate::logic::Weight;
use crate::ui::styles::*;

#[component]
pub fn ControlPanel(selected_coin: Signal<usize>, selected_weight: Signal<Weight>) -> Element {
    rsx! {
        div { class: "max-w-5xl mx-auto mb-16",
            div { class: "flex justify-center flex-nowrap gap-1 md:gap-3 mb-12",
                for i in 1..=12 {
                    CoinButton { 
                        id: i, 
                        selected: selected_coin() == i, 
                        on_click: move |_| { 
                            selected_coin.set(i); 
                            selected_weight.set(Weight::Normal); 
                        } 
                    }
                }
                CoinButton { 
                    id: 0, 
                    selected: selected_coin() == 0, 
                    on_click: move |_| { 
                        selected_coin.set(0); 
                        selected_weight.set(Weight::Normal); 
                    },
                    label: "None"
                }
            }

            if selected_coin() != 0 {
                WeightSelector { selected_weight: selected_weight }
            } else {
                div { class: "h-20" }
            }
        }
    }
}

#[component]
pub fn CoinButton(id: usize, selected: bool, on_click: EventHandler<MouseEvent>, label: Option<&'static str>) -> Element {
    let size_class = if label.is_some() { "px-4 md:px-6 h-10 md:h-16 rounded-full uppercase tracking-tight ml-2 md:ml-4" } 
                     else { "min-w-[40px] w-10 h-10 md:min-w-[64px] md:w-16 md:h-16 aspect-square rounded-full text-lg md:text-xl" };
    
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
                    class: "absolute -top-2 -right-2 w-6 h-6 bg-white rounded-full flex items-center justify-center text-xs text-indigo-600 font-black border-2 border-indigo-600 z-10", 
                    "!" 
                }
            }
        }
    }
}

#[component]
pub fn WeightSelector(selected_weight: Signal<Weight>) -> Element {
    let heavy_class = if selected_weight() == Weight::Heavy { BTN_WEIGHT_HEAVY } else { BTN_WEIGHT_HEAVY_INACTIVE };
    let light_class = if selected_weight() == Weight::Light { BTN_WEIGHT_LIGHT } else { BTN_WEIGHT_LIGHT_INACTIVE };

    rsx! {
        div { class: "h-20 flex justify-center items-center",
            div { class: "flex justify-center gap-6 animate-in fade-in slide-in-from-top-4 w-full max-w-md",
                button {
                    class: "{BTN_WEIGHT_BASE} {heavy_class}",
                    onclick: move |_| selected_weight.set(Weight::Heavy),
                    "HEAVIER"
                }
                button {
                    class: "{BTN_WEIGHT_BASE} {light_class}",
                    onclick: move |_| selected_weight.set(Weight::Light),
                    "LIGHTER"
                }
            }
        }
    }
}
