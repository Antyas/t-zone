pub struct Game {
  pub state: GameState
}

impl Game {
  pub fn new(state: &mut GameState) -> Self {
    Self { state }
  }
  pub fn start() {
    println!("Добро пожаловать");

    loop {
      self.component.run(self);
      Term::stdout().clear_screen().expect("Ошибка отчистки экрана");
    }
  }
}