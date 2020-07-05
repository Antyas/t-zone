mod game;
mod hero;

use game::Game;

fn main() {
  let mut game = Game::new();
  game.start();
}