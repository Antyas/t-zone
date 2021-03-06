mod component;
mod core;
mod entity;
mod content;

use crate::core::{Controller, Game, Store};

fn main() {
  let mut store = Store::new();
  let mut controller = Controller::new();
  Game::start(&mut store, &mut controller);
}
