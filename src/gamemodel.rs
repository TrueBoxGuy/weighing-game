use serde::{Deserialize, Serialize};
use crate::logic::{Weight, Outcome, Weighing, get_outcome};


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameConfig {
    pub num_coins: usize,
    pub bad_coins: BadCoinConstraints,
    pub deviation_min_factor: f64,
    pub deviation_max_factor: f64,
    /// Number of distinct weight groups for bad coins. None means infinity (each coin unique).
    pub grouping: Option<usize>,
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
        
        // Pre-calculate log bounds for log-uniform sampling
        let ln_min = config.deviation_min_factor.ln();
        let ln_max = config.deviation_max_factor.ln();

        // Helper to generate a valid non-1.0 factor using log-uniform distribution
        let gen_valid_factor = |rng: &mut rand::rngs::ThreadRng| {
            for _ in 0..10 {
                // Sample uniformly in log space: [ln(min), ln(max)]
                let r = rng.random_range(ln_min..=ln_max);
                // Convert back to linear space: factor = exp(r)
                let f = r.exp();
                
                if (f - 1.0).abs() > f64::EPSILON {
                    return f;
                }
            }
            // Fallback if we fail to generate a valid weight after 10 attempts
            // (e.g. if range is extremely tight around 1.0)
            1.1
        };

        // Limit bad coin weights to 'grouping' number of distinct values
        let available_factors: Vec<f64> = if let Some(g) = config.grouping {
            (0..g).map(|_| gen_valid_factor(&mut rng)).collect()
        } else {
            vec![]
        };

        for i in 0..num_bad {
            let idx = indices[i];
            let factor = if let Some(g) = config.grouping {
                // Pick from pre-generated groups
                available_factors[rng.random_range(0..g)]
            } else {
                // Generate unique factor
                gen_valid_factor(&mut rng)
            };
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
