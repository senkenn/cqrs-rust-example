#[derive(Clone)]
pub struct Student {
  pub id: u32,
  pub name: String,
}

impl Student {
  pub fn new(id: u32, name: String) -> Self {
      Self { id, name }
  }
}
