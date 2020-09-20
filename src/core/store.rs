use crate::entity::Hero;

pub struct Store {
  pub hero: Hero,
}

impl Store {
  pub fn new() -> Self {
    let hero: Hero = Default::default();
    Self { hero }
  }
}
