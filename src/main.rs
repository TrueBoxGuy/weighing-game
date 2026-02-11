use std::rc::Rc;
use dioxus::prelude::*;
use crate::logic::*;
use crate::ui::*;

pub mod logic;
pub mod ui;

fn main() {
    dioxus::launch(app);
}



fn app() -> Element {
    let selected_coin = use_signal(|| 0usize); 
    let selected_weight = use_signal(|| Weight::Normal);

    let display_path = use_signal(|| Vec::<(Weighing, Outcome)>::new());
    let visible_steps = use_signal(|| 0usize);
    
    let tree = use_memo(|| Rc::new(build_tree()));

    let target_path = use_simulation(tree, selected_coin, selected_weight);

    use_animation_controller(target_path, display_path, visible_steps);

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
        div { class: "{APP_CONTAINER}",
            Header {}
            
            ControlPanel {
                selected_coin: selected_coin,
                selected_weight: selected_weight
            }

            WeighingDisplay {
                path: display_path(),
                visible_steps: visible_steps(),
                selected_coin: selected_coin(),
                selected_weight: selected_weight()
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
}


