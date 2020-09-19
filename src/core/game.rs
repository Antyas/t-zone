use crate::component::{Component, MainMenu};
use crate::core::Store;
use console::Term;

pub struct Game {}

impl Game {
  pub fn start(store: &mut Store) {
    println!("Добро пожаловать");

    let mut component: Box<dyn Component> = Box::new(MainMenu::default());

    loop {
      component = component.run(store);
      Term::stdout()
        .clear_screen()
        .expect("Ошибка отчистки экрана");
    }
  }
}
