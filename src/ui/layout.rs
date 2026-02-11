use std::rc::Rc;
use dioxus::prelude::*;
use crate::logic::*;
use crate::ui::styles::*;
use crate::ui::visuals::TreeView;

#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "{HEADER_CONTAINER}",
            h1 { class: "{HEADER_H1}", 
                "12 COINS PUZZLE" 
            }
            p { class: "text-gray-400 text-sm uppercase tracking-widest", "Find the forgery in 3 weighings" }
        }
    }
}

#[component]
pub fn VisualizationFooter(tree: Rc<Node>, path: Vec<(Weighing, Outcome)>) -> Element {
    rsx! {
        footer { class: "max-w-full mx-auto pt-12 border-t border-neutral-800 bg-[#1a1a1a] min-h-[500px]",
            h3 { class: "text-center text-neutral-600 text-xs font-bold uppercase tracking-[0.5em] mb-12", "System Decision Tree" }
            div { class: "flex justify-center overflow-x-auto pb-24 px-8",
                svg { width: "100%", height: "auto", view_box: "0 0 1200 450", preserve_aspect_ratio: "xMidYMid meet",
                    TreeView { 
                        node: (*tree).clone(), 
                        path: path, 
                        x: 600.0, y: 50.0, width: 380.0, depth: 0 
                    }
                }
            }
        }
    }
}
