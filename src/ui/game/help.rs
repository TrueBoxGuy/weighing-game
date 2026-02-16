use dioxus::prelude::*;
use crate::ui::styles::*;

#[component]
pub fn HelpButton() -> Element {
    let mut show_help = use_signal(|| false);

    rsx! {
        button {
            class: "flex items-center gap-2 px-4 py-2 bg-neutral-800 hover:bg-neutral-700 text-neutral-400 hover:text-white rounded-lg transition-colors font-bold text-sm border border-neutral-700",
            onclick: move |_| show_help.set(true),
            span { class: "font-serif text-lg font-bold", "?" }
            span { class: "hidden md:inline", "HOW TO PLAY" }
        }

        if show_help() {
            div { 
                class: "{OVERLAY_CONTAINER} p-4", // Add padding for mobile edges
                onclick: move |_| show_help.set(false),
                div { 
                    // Manually re-implementing OVERLAY_BOX styles but with correct width and mobile scrolling
                    class: "bg-[#2a2a2a] rounded-3xl text-left relative w-full max-w-5xl px-6 md:px-12 py-6 md:py-8 shadow-2xl max-h-[90vh] overflow-y-auto",
                    onclick: move |e| e.stop_propagation(),
                    
                    div { class: "flex justify-between items-center mb-6",
                        h2 { class: "text-3xl font-black text-white tracking-tight", "HOW TO PLAY" }
                        button {
                            class: "text-neutral-400 hover:text-white bg-neutral-800/50 hover:bg-neutral-700 rounded-full w-8 h-8 flex items-center justify-center transition-colors",
                            onclick: move |_| show_help.set(false),
                            "✕"
                        }
                    }
                    
                    div { class: "space-y-8 text-neutral-300 text-base leading-relaxed",
                        
                        // 1. Objective
                        div { 
                            class: "bg-neutral-800/50 p-6 rounded-xl border border-neutral-700",
                            h3 { class: "text-white font-bold text-lg mb-2 flex items-center gap-2", 
                                span { class: "material-symbols-outlined text-yellow-400", "flag" }
                                "Objective" 
                            }
                            p { 
                                "Find the counterfeit coin hidden among the real ones using the balance scale. "
                                "Your goal is to " span { class: "text-white font-bold underline decoration-indigo-500 decoration-2 underline-offset-2", "minimize the number of weighings" } "."
                            }
                        }

                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                            // 2. Controls
                            div {
                                h3 { class: "text-white font-bold text-lg mb-2", "Controls" }
                                ul { class: "list-disc pl-5 space-y-2 marker:text-indigo-500",
                                    li { "Click coins to select them." }
                                    li { "Click Left/Right Pan to place selected coins." }
                                    li { "Click 'Weigh' to compare." }
                                }
                            }

                            // 3. Accusations
                            div {
                                h3 { class: "text-white font-bold text-lg mb-2", "Identifying the Fake" }
                                p { class: "mb-2",
                                    "Once you know the answer, use the " span { class: "text-white font-bold", "Make Accusation" } " box."
                                }
                                p { 
                                    "Click a coin to cycle colors: "
                                    span { class: "text-red-400 font-bold", "Heavy" } " → "
                                    span { class: "text-blue-400 font-bold", "Light" } " → Normal."
                                }
                            }
                        }

                        // 4. Configuration
                        div {
                            h3 { class: "text-white font-bold text-lg mb-2", "Game Configuration" }
                            div { class: "grid grid-cols-1 md:grid-cols-3 gap-4 text-sm bg-neutral-800 p-4 rounded-xl",
                                div { 
                                    span { class: "text-neutral-500 block text-xs uppercase font-bold tracking-wider mb-1", "Total Coins" } 
                                    "Choose between 3 and 20 coins." 
                                }
                                div { 
                                    span { class: "text-neutral-500 block text-xs uppercase font-bold tracking-wider mb-1", "Bad Coins" } 
                                    "Set how many counterfeits can exist." 
                                }
                                div { 
                                    span { class: "text-neutral-500 block text-xs uppercase font-bold tracking-wider mb-1", "Deviation & Grouping" } 
                                    "How much heavier/lighter a fake is."
                                    div { class: "text-xs text-neutral-400 mt-1 leading-tight", "(Grouping: Number of distinct fake weights)" }
                                }
                            }
                        }

                        // 5. Solver Mode Tip (Footer)
                        div { class: "pt-6 border-t border-neutral-700",
                            p { class: "text-center text-neutral-400",
                                span { class: "font-bold text-emerald-400", "Tip: " }
                                "Switch to " span { class: "text-white font-bold", "Solver Mode" } 
                                " to visualize the strategy for the classic 12 coin problem."
                            }
                        }
                    }

                    div { class: "mt-8 text-center",
                        button {
                            class: "{OVERLAY_BTN} min-w-[200px] shadow-xl hover:shadow-indigo-500/20",
                            onclick: move |_| show_help.set(false),
                            "Got it"
                        }
                    }
                }
            }
        }
    }
}
