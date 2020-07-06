mod game;
mod hero;
mod components;

use game::Game;

fn main() {
  let mut game = Game::new();
  game.start();
}