use crate::core::{Controller, Store};

pub struct Game {}

impl Game {
  pub fn start(store: &mut Store, controller: &mut Controller) {
    loop {
      controller.clear();
      controller.step(store);
    }
  }
}
