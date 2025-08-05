mod object;

use object::class::ClassName;

fn main() {
    let instance = ClassName::new(10);
    instance.public_method();
}