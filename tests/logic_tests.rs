use weighing_game::logic::*;
use weighing_game::game::*;


#[test]
fn test_get_outcome() {
    let mut weights = vec![Weight::NORMAL; 3];
    let w = Weighing::new(&[1], &[2]);

    // Case 1: Balanced
    assert_eq!(get_outcome(&w, &weights), Some(Outcome::Balanced));

    // Case 2: 1 is Heavy
    weights[0] = Weight::HEAVY;
    assert_eq!(get_outcome(&w, &weights), Some(Outcome::LeftHeavy));

    // Case 3: 1 is Normal, 2 is Light (makes Left heavier relative to right)
    weights[0] = Weight::NORMAL;
    weights[1] = Weight::LIGHT;
    assert_eq!(get_outcome(&w, &weights), Some(Outcome::LeftHeavy));

    // Case 4: 2 is Heavy
    weights[1] = Weight::HEAVY;
    assert_eq!(get_outcome(&w, &weights), Some(Outcome::RightHeavy));
}

#[test]
fn test_multiple_bad_coins() {
    // 1 Heavy, 2 Heavy. Left(1), Right(2). Balanced?
    let mut weights = vec![Weight::NORMAL; 5];
    weights[0] = Weight::HEAVY;
    weights[1] = Weight::HEAVY;
    
    let w = Weighing::new(&[1], &[2]);
    assert_eq!(get_outcome(&w, &weights), Some(Outcome::Balanced));
    
    // 1 Heavy, 2 Light. Left(1) vs Right(2). 
    // Left +1, Right -1. Left > Right.
    weights[1] = Weight::LIGHT;
    assert_eq!(get_outcome(&w, &weights), Some(Outcome::LeftHeavy));
}

#[test]
fn test_variable_weights() {
    let mut weights = vec![Weight::NORMAL; 4];
    weights[0] = Weight(2); // +2
    weights[1] = Weight(-1); // -1
    
    // Left: 1, 2 -> (+2) + (-1) = +1
    // Right: 3, 4 -> 0 + 0 = 0
    // LeftHeavy
    let w = Weighing::new(&[1, 2], &[3, 4]);
    assert_eq!(get_outcome(&w, &weights), Some(Outcome::LeftHeavy));
}

#[test]
fn test_game_state_init() {
    let config = GameConfig { 
        num_coins: 10, 
        bad_coins: BadCoinConstraints { min: 1, max: 2 },
        deviation_min: 1,
        deviation_max: 3,
    };
    let state = GameState::new(config.clone());
    
    assert_eq!(state.config.num_coins, 10);
    
    let bad_count = state.true_weights.iter().filter(|&&w| !w.is_normal()).count();
    assert!(bad_count >= 1 && bad_count <= 2);
    
    // Check if any bad coin has magnitude > 1 if possible
    // (This is probabilistic, but we check if logic runs)
}
