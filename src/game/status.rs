#[derive(Debug)]
pub enum GameStatus {
  RootMenu,
  LoadGameMenu,
  StartNewGame,
  Game
}

impl Default for GameStatus {
  fn default() -> Self{ GameStatus::RootMenu }
}