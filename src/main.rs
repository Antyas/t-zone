mod core;

use crate::core::{ Game, GameState };

fn main() {
  let mut state = GameState::new();
  let mut game = Game::new(&mut state);
  game.start();
}