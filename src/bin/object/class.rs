pub struct ClassName {
    field: i32,
}
impl ClassName {
    pub fn new(field: i32) -> ClassName {
        ClassName { field }
    }
    pub fn public_method(&self) {
        println!(
            "This is a public method with field: {}",
            self.private_method()
        );
    }
    fn private_method(&self) -> i32 {
        self.field * 2
    }
}
