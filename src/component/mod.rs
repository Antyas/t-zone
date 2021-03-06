mod main_menu;

use crate::core::Store;
pub use main_menu::MainMenu;

pub trait Component {
  fn draw(&mut self, state: &mut Store) -> Box<dyn Component>;
}
