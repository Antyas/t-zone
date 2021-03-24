#[derive(Debug)]
pub enum Race {
  Human(String)
}

impl Default for Race {
  fn default() -> Self {
    Race::Human(String::from("Человек"))
  }
}