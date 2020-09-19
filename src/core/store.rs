use crate::component::{Component, MainMenu};
use crate::entity::Hero;

pub struct Store {
  pub hero: Hero,
  pub component: Box<dyn Component>,
}

impl Store {
  pub fn new() -> Self {
    let hero: Hero = Default::default();
    let component = Box::new(MainMenu::default());
    Self { hero, component }
  }
}
