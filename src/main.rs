use rusty_engine::prelude::*;

struct GameState {}

fn main() {
    let mut game = Game::new();

    game.add_logic(game_logic);

    // initialize game state
    let initial_game_state = GameState {};
    game.run(initial_game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {}
