use crate::component::{Component, MainMenu};
use crate::core::Store;
use console::Term;

pub struct Controller {
  component: Box<dyn Component>,
  term_out: Term,
}

impl Controller {
  pub fn new() -> Self {
    let component = Box::new(MainMenu::default());
    let term_out = Term::stdout();
    Self {
      component,
      term_out,
    }
  }

  pub fn clear(&mut self) {
    self
      .term_out
      .clear_screen()
      .expect("Ошибка отчистки экрана");
  }

  pub fn step(&mut self, store: &mut Store) {
    self.component = self.component.run(store);
  }
}
