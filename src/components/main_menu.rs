use dialoguer::Select;
use crate::objects::{ Hero, Game };
use crate::components::Component;

pub enum MainMenu {
  Root,
  LoadGame,
  NewGame
}

impl Default for MainMenu {
  fn default() -> Self{ MainMenu::Root }
}

impl Component for MainMenu {
  fn run(&mut self, game: &mut Game) {
    loop {
      match self {
        MainMenu::Root => Box::new(self.root()),
        MainMenu::NewGame => Box::new(self.new_game(game)),
        _ => Box::new(self.root())
      };
    }
  }
}

impl MainMenu {
  fn root(&mut self) {
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
      0 => *self = MainMenu::NewGame,
      1 => *self = MainMenu::LoadGame,
      _ => *self = MainMenu::Root
    };
  }

  fn new_game(&mut self, game: &mut Game) {
    game.hero.init();
  }
}