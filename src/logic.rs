use std::rc::Rc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct Weight(pub f64);

impl Weight {
    pub const NORMAL: Self = Self(1.0);
    // These are just defaults, actual values will be dynamic
    pub const HEAVY: Self = Self(1.1); 
    pub const LIGHT: Self = Self(0.9);

    pub fn is_heavy(&self) -> bool {
        self.0 > 1.0 + f64::EPSILON
    }

    pub fn is_light(&self) -> bool {
        self.0 < 1.0 - f64::EPSILON
    }

    pub fn is_normal(&self) -> bool {
        (self.0 - 1.0).abs() < f64::EPSILON
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Outcome {
    Balanced,
    LeftHeavy,
    RightHeavy,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Weighing {
    pub left: Vec<usize>,
    pub right: Vec<usize>,
}

impl Weighing {
    pub fn new(left: &[usize], right: &[usize]) -> Self {
        Self {
            left: left.to_vec(),
            right: right.to_vec(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Decision(Weighing, Rc<OutcomeNodes>),
    Conclusion(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct OutcomeNodes {
    pub balanced: Node,
    pub left_heavy: Node,
    pub right_heavy: Node,
}

pub fn build_tree() -> Node {
    let c = |s: &str| Node::Conclusion(s.to_string());
    
    let decision = |left: &[usize], right: &[usize], l: Node, b: Node, r: Node| {
        Node::Decision(
            Weighing::new(left, right),
            Rc::new(OutcomeNodes { left_heavy: l, balanced: b, right_heavy: r })
        )
    };

    let w3_heavy_9_10_11 = decision(&[9], &[10], c("9H"), c("11H"), c("10H"));
    let w3_light_9_10_11 = decision(&[9], &[10], c("10L"), c("11L"), c("9L"));
    let w3_12 = decision(&[12], &[1], c("12H"), c("None"), c("12L"));
    let branch_balanced = decision(&[9, 10, 11], &[1, 2, 3], w3_heavy_9_10_11, w3_12, w3_light_9_10_11);

    let w3_l_l = decision(&[1], &[2], c("1H"), c("6L"), c("2H"));
    let w3_l_b = decision(&[7], &[8], c("8L"), c("4H"), c("7L"));
    let w3_l_r = decision(&[5], &[12], c("N/A"), c("3H"), c("5L"));
    let branch_left = decision(&[1, 2, 5], &[3, 6, 12], w3_l_l, w3_l_b, w3_l_r);

    let w3_r_l = decision(&[5], &[12], c("5H"), c("3L"), c("N/A"));
    let w3_r_b = decision(&[7], &[8], c("7H"), c("4L"), c("8H"));
    let w3_r_r = decision(&[1], &[2], c("2L"), c("6H"), c("1L"));
    let branch_right = decision(&[1, 2, 5], &[3, 6, 12], w3_r_l, w3_r_b, w3_r_r);

    decision(&[1, 2, 3, 4], &[5, 6, 7, 8], branch_left, branch_balanced, branch_right)
}



// Update get_outcome to use generic weights
pub fn get_outcome(weighing: &Weighing, weights: &[Weight]) -> Option<Outcome> {
    let mut left_weight = 0.0;
    let mut right_weight = 0.0;

    for &coin_idx in &weighing.left {
         // Coins are 1-indexed in UI/Weighing, but 0-indexed in weights vec
         if coin_idx > 0 && coin_idx <= weights.len() {
             left_weight += weights[coin_idx - 1].0;
         }
    }

    for &coin_idx in &weighing.right {
         if coin_idx > 0 && coin_idx <= weights.len() {
             right_weight += weights[coin_idx - 1].0;
         }
    }

    if left_weight > right_weight + f64::EPSILON {
        Some(Outcome::LeftHeavy)
    } else if right_weight > left_weight + f64::EPSILON {
        Some(Outcome::RightHeavy)
    } else {
        Some(Outcome::Balanced)
    }
}



pub mod simulation;
pub use simulation::*;
