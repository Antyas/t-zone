use crate::component::{Component, MainMenu};
use crate::core::Store;
use console::Term;

pub struct Controller {
  component: Box<dyn Component>,
}

impl Controller {
  pub fn new() -> Self {
    let component = Box::new(MainMenu::default());

    Self {
      component,
    }
  }

  pub fn clear() {
    Term::stdout()
      .clear_screen()
      .expect("Ошибка отчистки экрана");
  }

  pub fn next(&mut self, store: &mut Store) {
    Self::clear();
    self.component = self.component.draw(store);
  }
}
