use crate::gamemodel::GameConfig;

pub fn generate_ai_prompt(config: &GameConfig) -> String {
    let grouping_str = if let Some(g) = config.grouping { g.to_string() } else { "Infinity".to_string() };
    format!(r#"I am playing a logic game where I need to find counterfeit coins using a balance scale.

Current Game Configuration:
- Total Coins: {}
- Bad Coins: {} to {}
- Deviation Factor: {} to {} (Multiplicative factor of weight. Normal coin = 1.0. Heavy > 1.0, Light < 1.0)
- Grouping: {} (The number of distinct weight values available for bad coins. Bad coins are "bucketed" into these values.)

Goal:
IDENTIFY all bad coins and their types (heavy/light) in the MINIMUM number of weighings.

Rules:
1. I have a set of coins (1 to {}), some of which may be counterfeit.
2. Counterfeit coins deviate from the normal weight by the Deviation Factor range. 
   - Example: Factor 1.2 means a heavy coin weighs 1.2x. Factor 0.8 means a light coin weighs 0.8x.
   - Grouping / Bucketing: If Grouping is N, exactly N valid deviation factors are generated. Each bad coin is randomly assigned one of these N factors. This acts as a bucket system where multiple bad coins may share the exact same weight. If Grouping is Infinity, each bad coin has a unique weight.
3. I can select coins to place on the left and right pans of a balance scale.
4. The scale will tell me if the left side is heavier, lighter, or balanced.

I will provide you with the history of my weighings. Please analyze the results to deduce the status of each coin.
IMPORTANT: For any counterfeit coins you identify, you MUST explicitly state if they are "Heavier" or "Lighter" than a normal coin.

The history format will be:
1. [Left Coins] vs [Right Coins] -> Outcome

You can ask me to "Copy History" at any time to get the latest state."#, 
        config.num_coins, 
        config.bad_coins.min, config.bad_coins.max,
        config.deviation_min_factor, config.deviation_max_factor,
        grouping_str,
        config.num_coins
    )
}
