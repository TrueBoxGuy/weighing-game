use dioxus::prelude::*;
use crate::ui::styles::*;

#[component]
pub fn ConfigInput(
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
