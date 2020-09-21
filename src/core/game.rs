use crate::core::{Controller, Store};

pub struct Game {}

impl Game {
  pub fn start(store: &mut Store, controller: &mut Controller) {
    controller.clear();

    loop {
      controller.step(store);
      controller.clear();
    }
  }
}
