use crate::component::Component;
use crate::core::Store;
use dialoguer::{Input, Select};

pub enum MainMenu {
  Root,
  LoadGame,
  NewGame,
}

impl Default for MainMenu {
  fn default() -> Self {
    MainMenu::Root
  }
}

impl Component for MainMenu {
  fn run(&mut self, store: &mut Store) -> Box<dyn Component> {
    let component = match self {
      MainMenu::Root => Box::new(self.root()),
      MainMenu::NewGame => Box::new(self.new_game(store)),
      _ => Box::new(self.root()),
    };
    component
  }
}

impl MainMenu {
  fn root(&mut self) -> Self {
    let items = ["Новая игра", "Загрузить сохранение"];

    let index: usize = Select::new()
      .with_prompt("Меню")
      .items(&items)
      .default(0)
      .interact()
      .expect("Ошибка выбора пункта меню");

    match index {
      0 => MainMenu::NewGame,
      1 => MainMenu::LoadGame,
      _ => MainMenu::Root,
    }
  }

  fn new_game(&mut self, store: &mut Store) -> Self {
    store.hero.name = Input::new()
      .with_prompt("Имя персоонажа")
      .interact()
      .expect("Ошибка заполнения имени");

    MainMenu::Root
  }
}
