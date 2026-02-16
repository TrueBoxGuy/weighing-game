use serde::{Deserialize, Serialize};
use crate::logic::{Weight, Outcome, Weighing, get_outcome};


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameConfig {
    pub num_coins: usize,
    pub bad_coins: BadCoinConstraints,
    pub deviation_min_factor: f64,
    pub deviation_max_factor: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BadCoinConstraints {
    pub min: usize,
    pub max: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    pub config: GameConfig,
    /// Hidden true weights of all coins.
    pub true_weights: Vec<Weight>,
    pub history: Vec<(Weighing, Outcome)>,
    #[serde(skip)]
    pub start_time: Option<web_time::Instant>,
}

impl GameState {
    pub fn new(config: GameConfig) -> Self {
        use rand::prelude::*;
        let mut rng = rand::rng();
        
        // Generate number of bad coins
        let num_bad = rng.random_range(config.bad_coins.min..=config.bad_coins.max);
        
        let mut weights = vec![Weight::NORMAL; config.num_coins];
        
        // Randomly select indices for bad coins
        let mut indices: Vec<usize> = (0..config.num_coins).collect();
        indices.shuffle(&mut rng);
        
        for i in 0..num_bad {
            let idx = indices[i];
            let factor = rng.random_range(config.deviation_min_factor..=config.deviation_max_factor);
            weights[idx] = Weight(factor);
        }

        Self {
            config,
            true_weights: weights,
            history: Vec::new(),
            start_time: Some(web_time::Instant::now()),
        }
    }
    
    pub fn weigh(&mut self, weighing: Weighing) -> Option<Outcome> {
        let outcome = get_outcome(&weighing, &self.true_weights);
        
        if let Some(o) = outcome {
            self.history.push((weighing, o));
        }
        
        outcome
    }
}
