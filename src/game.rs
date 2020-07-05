mod status;

// use std::io::{ stdin };
use crate::hero::Hero;
use status::GameStatus;
use dialoguer::Select;
use console::Term;

pub struct Game {
  hero: Hero,
  status: GameStatus
}

impl Game {
  pub fn new() -> Game {
    let hero: Hero = Default::default();
    let status: GameStatus = Default::default();

    Game { hero, status }
  }

  pub fn start(&mut self) {
    println!("Добро пожаловать в зону");

    loop {
      match self.status {
        GameStatus::RootMenu => self.root_menu(),
        GameStatus::StartNewGame => self.create_new_hero(),
        GameStatus::LoadGameMenu => (),
        GameStatus::Game => ()
      }
      Term::stdout().clear_screen().expect("Ошибка");
      println!("{:?}", self.status);
    }
  }

  fn root_menu(&mut self) {
    let items = [
      "Новая игра",
      "Загрузить сохранение"
    ];

    let index: usize = Select::new()
      .with_prompt("Меню")
      .items(&items)
      .default(0)
      .interact()
      .expect("Ошибка выбора пункта меню");

    match index {
      0 => self.status = GameStatus::StartNewGame,
      1 => self.status = GameStatus::LoadGameMenu,
      _ => ()
    }
  }

  fn create_new_hero(&mut self) {
    self.hero = Hero::create();
  }
}


// let mut input = String::new();

// stdin().read_line(&mut input)
//   .expect("Failed to read line");