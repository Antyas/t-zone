use dialoguer::Input;

#[derive(Default)]
pub struct Hero {
  name: String
}

impl Hero {
  pub fn init(&mut self) {
    self.read_name();
  }

  fn read_name(&mut self) {
    self.name = Input::new()
      .with_prompt("Имя персоонажа")
      .interact()
      .expect("Ошибка заполнения имени");
  }
  
}