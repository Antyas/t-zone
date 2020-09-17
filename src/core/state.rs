mod components;
mod entity;

use crate::components::{ Component, MainMenu };
use crate::entity::{ Hero };


pub struct GameState {
  pub hero: Hero,
  pub component: Box<dyn Component>
}

impl GameState {
  pub fn new() -> Self {
    let hero: Hero = Default::default();
    let component = Box::new(MainMenu::default());
    Self { hero, component }
  }
}