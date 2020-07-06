#[derive(Debug)]
pub enum Global {
  MainMenu,
  Game
}

impl Default for Global {
  fn default() -> Self{ Global::MainMenu }
}

