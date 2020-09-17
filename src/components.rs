mod main_menu;

pub use main_menu::MainMenu;
use crate::objects::Game;

pub trait Component {
  fn run(&mut self, game: &mut Game);
}