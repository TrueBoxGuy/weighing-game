use dioxus::prelude::*;
use crate::logic::*;
use crate::ui::styles::*;

#[component]
pub fn HistoryLog(history: Vec<(Weighing, Outcome)>) -> Element {
    let mut page = use_signal(|| 0usize);
    let items_per_page = 5;
    let total_pages = (history.len() + items_per_page - 1) / items_per_page;
    let current_page = if total_pages > 0 {
         if *page.read() >= total_pages {
             page.set(total_pages - 1);
        }
        *page.read()
    } else {
        0
    };

    rsx! {
        div { class: "bg-[#1a1a1a] rounded-2xl border border-neutral-800 flex flex-col relative",
            div { class: "p-6 pb-2 bg-[#1a1a1a] z-10 border-b border-neutral-800/50 flex justify-between items-center",
                h3 { class: "text-neutral-400 font-bold text-xs uppercase tracking-widest", "HISTORY" }
                if total_pages > 1 {
                    div { class: "flex gap-2 text-xs font-mono",
                        button { 
                            class: "px-2 py-1 bg-neutral-800 hover:bg-neutral-700 rounded text-neutral-400 disabled:opacity-30",
                            disabled: current_page == 0,
                            onclick: move |_| page.set(page() - 1),
                            "Prev"
                        }
                        span { class: "text-neutral-500 py-1", "{current_page + 1} / {total_pages}" }
                        button { 
                            class: "px-2 py-1 bg-neutral-800 hover:bg-neutral-700 rounded text-neutral-400 disabled:opacity-30",
                            disabled: current_page >= total_pages - 1,
                            onclick: move |_| page.set(page() + 1),
                            "Next"
                        }
                    }
                }
            }
            
            div { class: "p-6 pt-2 space-y-3 min-h-[300px]",
                if history.is_empty() {
                    div { class: "text-neutral-600 text-sm text-center py-8", "No weighings yet" }
                } else {
                    for (i, (w, o)) in history.iter().enumerate().rev().skip(current_page * items_per_page).take(items_per_page) {
                        div { class: "{HISTORY_ITEM}",
                            div { class: "text-xs font-mono text-neutral-400", "#{i+1}" }
                            div { class: "flex-1 px-4 flex justify-between text-sm font-bold",
                                span { class: "text-neutral-300", "{w.left:?}" }
                                span { class: "text-neutral-500", "vs" }
                                span { class: "text-neutral-300", "{w.right:?}" }
                            }
                            div { 
                                match o {
                                    Outcome::Balanced => rsx! { span { class: "text-emerald-400 font-bold text-xs", "=" } },
                                    Outcome::LeftHeavy => rsx! { span { class: "text-red-400 font-bold text-xs", "L > R" } },
                                    Outcome::RightHeavy => rsx! { span { class: "text-blue-400 font-bold text-xs", "R > L" } },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
