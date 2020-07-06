use dialoguer::Select;
use crate::components::Global;

pub enum MainMenu {
  Root,
  LoadGame,
  NewGame
}

impl Default for MainMenu {
  fn default() -> Self{ MainMenu::Root }
}

impl MainMenu {
  pub fn run(&mut self) {
    loop {
      match self {
        MainMenu::Root => self.root(),
        _ => ()
      }
    }
  }

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
      0 => *self = MainMenu::LoadGame,
      1 => *self = MainMenu::NewGame,
      _ => ()
    };
  }
}