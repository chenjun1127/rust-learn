pub struct MyStruct {
  name: String,
  age: u32,
}
impl MyStruct {
  pub fn new(name: String, age: u32) -> Self {
      MyStruct { name, age }
  }

  pub fn some_function(&self) {
      // 实现函数逻辑
      println!("Name: {}, Age: {}", self.name, self.age);
  }
}
