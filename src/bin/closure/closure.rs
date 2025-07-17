fn main() {
    fn func(x: i32) -> i32 {
        x + 1
    }
    println!("func(1) = {}", func(1));
    let closure = |x: i32| -> i32 { x + 1 };
    println!("closure(1) = {}", closure(1));
    let closure_inferred = |x| x + 1;
    println!("closure_inferred(1) = {}", closure_inferred(1));
    let one = || 1;
    println!("closure returning one: {}", one());
}
