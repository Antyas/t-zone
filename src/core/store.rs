use crate::entity::Character;

#[derive(Debug)]
pub struct Store {
  pub hero: Character,
}

impl Store {
  pub fn new() -> Self {
    let hero: Character = Default::default();
    Self { hero }
  }
}
