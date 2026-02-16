use dioxus::prelude::*;
use crate::logic::*;

#[component]
pub fn TreeView(node: Node, path: Vec<(Weighing, Outcome)>, x: f32, y: f32, width: f32, depth: usize) -> Element {
    let is_active = !path.is_empty();
    let current_outcome = path.first().map(|(_, o)| *o);
    
    match node {
        Node::Decision(w, outcomes) => {
            let next_path = if is_active { path[1..].to_vec() } else { vec![] };
            let variants = [
                (Outcome::LeftHeavy, &outcomes.left_heavy, -width, "#ef4444"),  
                (Outcome::Balanced, &outcomes.balanced, 0.0, "#10b981"),        
                (Outcome::RightHeavy, &outcomes.right_heavy, width, "#3b82f6"), 
            ];

            rsx! {
                g {
                    // Recursive Edges
                    for (out, child, offset, color) in variants.iter() {
                        TreeEdge {
                            outcome: *out,
                            child_node: (**child).clone(),
                            current_outcome,
                            next_path: next_path.clone(),
                            is_active,
                            color,
                            x, y, offset: *offset, width, depth
                        }
                    }

                    // Node Label ("{...} vs {...}")
                    TreeLabel { x, y, left: w.left, right: w.right }

                    // Circle Indicator ("W1", "W2"...)
                    NodeIndicator { x, y, depth, is_active }
                }
            }
        }
        Node::Conclusion(s) => rsx! { ConclusionNode { x, y, text: s, is_active } }
    }
}

#[component]
fn TreeEdge(
    outcome: Outcome, 
    child_node: Node, 
    current_outcome: Option<Outcome>, 
    next_path: Vec<(Weighing, Outcome)>,
    is_active: bool,
    color: &'static str,
    x: f32, y: f32, offset: f32, width: f32, depth: usize
) -> Element {
    // Skip rendering impossible branches (N/A)
    if let Node::Conclusion(s) = &child_node {
        if s == "N/A" { return rsx! { g { key: "{outcome:?}" } }; }
    }

    let is_on_path = is_active && current_outcome == Some(outcome);
    let stroke = if is_on_path { color } else { "#525252" };
    let stroke_width = if is_on_path { 3.0 } else { 1.5 };
    let y_step = 100.0;

    rsx! {
        g { key: "{outcome:?}",
            line { 
                x1: "{x}", y1: "{y}", x2: "{x + offset}", y2: "{y + y_step}", 
                stroke: "{stroke}", 
                stroke_width: "{stroke_width}" 
            }
            TreeView { 
                node: child_node, 
                path: if is_on_path { next_path } else { vec![] }, 
                x: x + offset, y: y + y_step, 
                width: width / 3.0, 
                depth: depth + 1 
            }
        }
    }
}

#[component]
fn TreeLabel(x: f32, y: f32, left: Vec<usize>, right: Vec<usize>) -> Element {
    let left_str = left.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");
    let right_str = right.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");
    let label = format!("{{ {} }} vs {{ {} }}", left_str, right_str);
    let label_width = (label.len() as f32) * 5.0 + 25.0; 

    rsx! {
        rect {
            x: "{x - label_width / 2.0}", y: "{y - 35.0}",
            width: "{label_width}", height: "20", rx: "4",
            fill: "#1f2937", stroke: "#4b5563", stroke_width: "1"
        }
        text {
            x: "{x}", y: "{y - 21.0}",
            text_anchor: "middle",
            fill: "#f3f4f6",
            font_size: "10",
            font_family: "monospace",
            font_weight: "bold",
            "{label}"
        }
    }
}

#[component]
fn NodeIndicator(x: f32, y: f32, depth: usize, is_active: bool) -> Element {
    let fill = if is_active { "#1e1b4b" } else { "#171717" };
    let stroke = if is_active { "#6366f1" } else { "#525252" };
    let text_fill = if is_active { "#fff" } else { "#a3a3a3" };

    rsx! {
        circle { 
            cx: "{x}", cy: "{y}", r: "12", 
            fill: "{fill}", 
            stroke: "{stroke}",
            stroke_width: "2" 
        }
        text { 
            x: "{x}", y: "{y + 4.0}", 
            text_anchor: "middle", 
            fill: "{text_fill}", 
            font_size: "10", 
            font_weight: "bold", 
            "W{depth+1}" 
        }
    }
}

#[component]
fn ConclusionNode(x: f32, y: f32, text: String, is_active: bool) -> Element {
    let color = if text.ends_with('H') { "#ef4444" } else if text.ends_with('L') { "#3b82f6" } else { "#10b981" };
    let is_impossible = text == "N/A" || text == "None";
    let final_color = if is_impossible { "#666" } else { color };
    
    let bg_stroke = if is_active { final_color } else { "#525252" };
    let text_fill = if is_active { "#fff" } else { "#d4d4d4" };

    rsx! {
        g {
            rect { 
                x: "{x - 15.0}", y: "{y - 10.0}", width: "30", height: "20", rx: "6", 
                fill: "#000", 
                stroke: "{bg_stroke}", 
                stroke_width: if is_active { "2" } else { "1" } 
            }
            text { 
                x: "{x}", y: "{y + 4.0}", 
                text_anchor: "middle", 
                fill: "{text_fill}", 
                font_size: "9", 
                font_weight: "bold", 
                "{text}" 
            }
        }
    }
}
