use dialoguer::Input;

#[derive(Default)]
pub struct Hero {
  name: String
}

impl Hero {
  pub fn create() -> Hero {
    Hero {
      name: Hero::read_name()
    }
  }

  fn read_name() -> String {
    let name: String = Input::new()
      .with_prompt("Имя персоонажа")
      .interact()
      .expect("Ошибка заполнения имени");

    name
  }
  
}