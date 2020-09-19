mod component;
mod core;
mod entity;

use crate::core::{Game, Store};

fn main() {
  let mut store = Store::new();
  Game::start(&mut store);
}
