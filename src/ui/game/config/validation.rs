use crate::gamemodel::GameConfig;

pub struct ConfigValidation {
    pub bad_min_valid: bool,
    pub bad_max_valid: bool,
    pub dev_min_valid: bool,
    pub dev_max_valid: bool,
    pub grouping_valid: bool,
    pub is_valid: bool,
}

const MIN_FACTOR: f64 = 0.1;
const MAX_FACTOR: f64 = 10.0;
const MAX_GROUPING: usize = 10_000;

fn is_valid_factor(f: f64) -> bool {
    f >= MIN_FACTOR && f <= MAX_FACTOR && (f - 1.0).abs() > 0.001
}

pub fn validate_config(c: &GameConfig) -> ConfigValidation {
    let bad_min = c.bad_coins.min <= c.bad_coins.max;
    // bad_max check: must be >= min, and <= num_coins (which is capped at 20, but the range input enforces that too)
    // The range input for bad_max is constrained by `num_coins`, so we just check basic sanity here.
    let bad_max = c.bad_coins.max >= c.bad_coins.min && c.bad_coins.max <= c.num_coins;
    
    let dev_min = is_valid_factor(c.deviation_min_factor) 
        && c.deviation_min_factor <= c.deviation_max_factor;
        
    let dev_max = is_valid_factor(c.deviation_max_factor) 
        && c.deviation_max_factor >= c.deviation_min_factor;
    
    let grouping_valid = c.grouping.map_or(true, |g| g > 0 && g <= MAX_GROUPING);

    let global = c.num_coins >= 3 && c.num_coins <= 20
        && bad_min && bad_max
        && dev_min && dev_max
        && grouping_valid;

    ConfigValidation {
        bad_min_valid: bad_min,
        bad_max_valid: bad_max,
        dev_min_valid: dev_min,
        dev_max_valid: dev_max,
        grouping_valid,
        is_valid: global,
    }
}
