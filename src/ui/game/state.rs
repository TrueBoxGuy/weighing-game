use dioxus::prelude::*;
use crate::logic::*;
use crate::gamemodel::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameStatus {
    Playing,
    Won(usize), // (attempts)
    Lost(usize), // (attempts)
}

#[derive(Clone, Copy, PartialEq)]
pub struct GameModeLogic {
    pub state: Signal<GameState>,
    pub current_weighing: Signal<Weighing>,
    pub status: Signal<GameStatus>,
}

impl GameModeLogic {
    // Helper to reset the state, essentially logic from the previous reset method
    pub fn reset(&mut self) {
        let config = self.state.read().config.clone();
        self.state.set(GameState::new(config));
        self.current_weighing.set(Weighing::new(&[], &[]));
        self.status.set(GameStatus::Playing);
    }
}

pub fn use_game_mode() -> GameModeLogic {
    let state = use_signal(|| GameState::new(GameConfig {
        num_coins: 5,
        bad_coins: BadCoinConstraints { min: 0, max: 1 },
        deviation_min_factor: 0.8,
        deviation_max_factor: 1.3,
    }));
    let current_weighing = use_signal(|| Weighing::new(&[], &[]));
    let status = use_signal(|| GameStatus::Playing);

    GameModeLogic {
        state,
        current_weighing,
        status,
    }
}
