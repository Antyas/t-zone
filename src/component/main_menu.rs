use crate::component::Component;
use crate::core::Store;
use crate::content::Race;
use dialoguer::{ Input, Select };

pub enum MainMenu {
  Root,
  LoadGame,
  NewGame,
  Debug,
}

impl Default for MainMenu {
  fn default() -> Self {
    MainMenu::Root
  }
}

impl Component for MainMenu {
  fn draw(&mut self, store: &mut Store) -> Box<dyn Component> {
    match self {
      MainMenu::Root => Box::new(self.root()),
      MainMenu::NewGame => Box::new(self.new_game(store)),
      MainMenu::Debug => Box::new(self.debug(store)),
      _ => Box::new(self.root()),
    }
  }
}

impl MainMenu {
  fn root(&mut self) -> Self {
    let items = ["Новая игра", "Загрузить сохранение", "Debug"];

    let index: usize = Select::new()
      .with_prompt("Меню")
      .items(&items)
      .default(0)
      .interact()
      .expect("Ошибка выбора пункта меню");

    match index {
      0 => MainMenu::NewGame,
      1 => MainMenu::LoadGame,
      2 => MainMenu::Debug,
      _ => MainMenu::Root,
    }
  }

  fn new_game(&mut self, store: &mut Store) -> Self {
    store.hero.name = Input::new()
      .with_prompt("Имя персоонажа")
      .interact()
      .expect("Ошибка заполнения имени");

    let races = ["Человек"];

    let race_index: usize = Select::new()
      .with_prompt("Раса персонажа")
      .items(&races)
      .default(0)
      .interact()
      .expect("Ошибка выбора расы");

    store.hero.race = match race_index {
      _ => Race::Human(String::from("Человек")),
    };

    MainMenu::Root
  }

  fn debug(&mut self, store: &mut Store) -> Self {
    println!("{:?}", store);

    let _: String = Input::new()
      .with_prompt("Ожидание ввода любого символа")
      .interact()
      .expect("Ошибка заполнения имени");

    MainMenu::Root
  }
}
