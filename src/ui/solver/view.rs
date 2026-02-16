use std::rc::Rc;
use dioxus::prelude::*;
use crate::logic::*;
use crate::ui::solver::{ControlPanel, WeighingDisplay, ConclusionDisplay, VisualizationFooter};
use crate::ui::common::Header;
use crate::ui::use_animation_controller;

#[component]
pub fn SolverMode() -> Element {
    // UI State (Instant updates)
    let mut ui_selected_coin = use_signal(|| 0usize); 
    let mut ui_selected_weight = use_signal(|| Weight::NORMAL);

    // Simulation State (Delayed updates)
    let mut sim_selected_coin = use_signal(|| 0usize); 
    let mut sim_selected_weight = use_signal(|| Weight::NORMAL);

    let display_path = use_signal(|| Vec::<(Weighing, Outcome)>::new());
    let mut visible_steps = use_signal(|| 0usize);
    
    let tree = use_memo(|| Rc::new(build_tree()));

    // Simulation drives off the SIM state
    let target_path = use_simulation(tree, sim_selected_coin, sim_selected_weight);

    // Animation controller observes target_path changes
    use_animation_controller(target_path, display_path, visible_steps);

    // Handlers
    let handle_coin_change = move |coin: usize| {
        ui_selected_coin.set(coin);
        ui_selected_weight.set(Weight::NORMAL); // Reset weight on coin change
        
        // Clear visualization immediately
        visible_steps.set(0); 
        
        // Delay simulation update
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(300).await;
            sim_selected_coin.set(coin);
            sim_selected_weight.set(Weight::NORMAL);
        });
    };

    let handle_weight_change = move |weight: Weight| {
        ui_selected_weight.set(weight);
         
        // Clear visualization immediately
        visible_steps.set(0); 
        
        // Delay simulation update
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(300).await;
            sim_selected_weight.set(weight);
        });
    };

    let conclusion = {
        let current_tree = tree();
        let mut curr: &Node = &*current_tree;
        for (_, outcome) in display_path.read().iter() {
            if let Node::Decision(_, outcomes) = curr {
                curr = match outcome {
                    Outcome::Balanced => &outcomes.balanced,
                    Outcome::LeftHeavy => &outcomes.left_heavy,
                    Outcome::RightHeavy => &outcomes.right_heavy,
                };
            }
        }
        if let Node::Conclusion(s) = curr { s.clone() } else { "".to_string() }
    };

    rsx! {
        Header {}
        
        ControlPanel {
            selected_coin: ui_selected_coin(),
            selected_weight: ui_selected_weight(),
            on_coin_change: handle_coin_change,
            on_weight_change: handle_weight_change,
        }

        WeighingDisplay {
            path: display_path(),
            visible_steps: visible_steps(),
            selected_coin: sim_selected_coin(),
            selected_weight: sim_selected_weight()
        }

        ConclusionDisplay {
            conclusion: conclusion,
            visible: visible_steps() >= display_path.read().len() && !display_path.read().is_empty()
        }

        VisualizationFooter {
            tree: tree(), 
            path: display_path.read().iter().take(visible_steps()).cloned().collect()
        }
    }
}
