pub enum Operation{
  Add,
  Subtract,
}
impl Operation{
  pub fn apply(&self, x: i32, y: i32) -> i32 {
    match self {
      Operation::Add => x + y,
      Operation::Subtract => x - y,
    }
  }
}