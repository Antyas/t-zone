use crate::hero::Hero;
use crate::components::Global;
use console::Term;

pub struct Game {
  hero: Hero,
  status: Global
}

impl Game {
  pub fn new() -> Game {
    let hero: Hero = Default::default();
    let status: Global = Default::default();

    Game { hero, status }
  }

  pub fn start(&mut self) {
    println!("Добро пожаловать в зону");

    loop {
      // self.status.run();
      Term::stdout().clear_screen().expect("Ошибка отчистки экрана");
      println!("{:?}", self.status);
    }
  }

  fn create_new_hero(&mut self) {
    self.hero = Hero::create();
  }
}


// let mut input = String::new();

// stdin().read_line(&mut input)
//   .expect("Failed to read line");