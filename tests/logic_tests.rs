use weighing_game::logic::*;
use weighing_game::gamemodel::*;

#[test]
fn test_get_outcome_balanced() {
    let weights = vec![Weight::NORMAL; 5];
    
    // Test 1: Balanced (Normal vs Normal)
    let w = Weighing::new(&[1], &[2]); 
    assert_eq!(get_outcome(&w, &weights), Some(Outcome::Balanced));
}

#[test]
fn test_get_outcome_left_heavy() {
    let mut weights = vec![Weight::NORMAL; 5];
    weights[0] = Weight::HEAVY; // Coin 1 is heavy
    
    // Left has heavy coin (Coin 1)
    let w = Weighing::new(&[1], &[2]);
    assert_eq!(get_outcome(&w, &weights), Some(Outcome::LeftHeavy));
}

#[test]
fn test_get_outcome_right_heavy() {
    let mut weights = vec![Weight::NORMAL; 5];
    weights[1] = Weight::HEAVY; // Coin 2 is heavy
    
    // Right has heavy coin (Coin 2)
    let w = Weighing::new(&[1], &[2]);
    assert_eq!(get_outcome(&w, &weights), Some(Outcome::RightHeavy));
}

#[test]
fn test_multiple_bad_coins() {
    // 1 Heavy, 2 Heavy. Left(1), Right(2). Balanced?
    let mut weights = vec![Weight::NORMAL; 5];
    weights[0] = Weight::HEAVY;
    weights[1] = Weight::HEAVY;
    
    let w = Weighing::new(&[1], &[2]);
    // 1.1 vs 1.1 -> Balanced
    assert_eq!(get_outcome(&w, &weights), Some(Outcome::Balanced));
    
    // 1 Heavy (1.1), 2 Light (0.9). Left(1) vs Right(2). 
    // Left 1.1, Right 0.9. Left > Right.
    weights[1] = Weight::LIGHT;
    assert_eq!(get_outcome(&w, &weights), Some(Outcome::LeftHeavy));
}

#[test]
fn test_variable_weights() {
    let mut weights = vec![Weight::NORMAL; 4];
    weights[0] = Weight(2.0); // Coin 1 = 2.0
    weights[1] = Weight(0.5); // Coin 2 = 0.5
    
    // Left: 1, 2 -> 2.0 + 0.5 = 2.5
    // Right: 3, 4 -> 1.0 + 1.0 = 2.0
    // LeftHeavy
    let w = Weighing::new(&[1, 2], &[3, 4]);
    assert_eq!(get_outcome(&w, &weights), Some(Outcome::LeftHeavy));
}

#[test]
fn test_game_state_init() {
    let config = GameConfig { 
        num_coins: 10, 
        bad_coins: BadCoinConstraints { min: 1, max: 2 },
        deviation_min_factor: 1.1,
        deviation_max_factor: 1.5,
        grouping: None,
    };
    let state = GameState::new(config.clone());
    
    assert_eq!(state.config.num_coins, 10);
    
    let bad_count = state.true_weights.iter().filter(|&&w| !w.is_normal()).count();
    assert!(bad_count >= 1 && bad_count <= 2);
}
