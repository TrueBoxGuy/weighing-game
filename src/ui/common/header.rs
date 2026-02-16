use dioxus::prelude::*;
use crate::ui::styles::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "{HEADER_CONTAINER}",
            h1 { class: "{HEADER_H1}", 
                "12 COINS PUZZLE" 
            }
            p { class: "{HEADER_SUBTITLE}", "Find the forgery in 3 weighings" }
        }
    }
}
