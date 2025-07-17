use std::mem;

fn main() {
    let a = Box::new(3);
    let consume = || {
        println!("{}", a);
        mem::drop(a);
    };
    consume(); // 消耗了该变量，所以该闭包只能调用一次。
    // consume();
}
