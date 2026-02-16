use std::rc::Rc;
use dioxus::prelude::*;
use crate::logic::*;

pub fn use_simulation(
    tree: Memo<Rc<Node>>, 
    selected_coin: Signal<usize>, 
    selected_weight: Signal<Weight>
) -> Memo<Vec<(Weighing, Outcome)>> {
    use_memo(move || {
        if selected_coin() != 0 && selected_weight() == Weight::NORMAL {
            return Vec::new();
        }
        
        let mut p = Vec::new();
        let current_tree = tree();
        let mut curr: &Node = &*current_tree;
        
        while let Node::Decision(weighing, outcomes) = curr {
            let outcome = if selected_coin() == 0 {
                 Outcome::Balanced 
            } else {
                 let mut weights = vec![Weight::NORMAL; 12];
                 if selected_coin() <= 12 {
                     weights[selected_coin() - 1] = selected_weight();
                 }
                 get_outcome(weighing, &weights).unwrap_or(Outcome::Balanced)
            };
            
            p.push((weighing.clone(), outcome));
            curr = match outcome {
                Outcome::Balanced => &outcomes.balanced,
                Outcome::LeftHeavy => &outcomes.left_heavy,
                Outcome::RightHeavy => &outcomes.right_heavy,
            };
        }
        if let Node::Conclusion(_) = curr {
            p.push((Weighing { left: vec![], right: vec![] }, Outcome::Balanced)); 
        }
        p
    })
}
