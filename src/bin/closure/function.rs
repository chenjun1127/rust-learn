fn main() {
    let closure = || {
        println!("I'm a closure!");
    };
    call_me(closure);
    call_me(function);
    call_me(|| println!("I'm another closure!"));
    println!("---------------------------");
    let mut count = 0;
    let add = || {
        count += 1; // 需要可变访问
        println!("count = {}", count);
    };
    call_fnmut(add);
    println!("---------------------------");
    let name = String::from("Rust");
    let consume = || {
        println!("consuming {}", name); // move 走了 `name`
        drop(name); // 显式释放
    };
    call_fnonce(consume);
}
fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("Hello, world!");
}

fn call_fnmut<F: FnMut()>(mut f: F) {
    f();
    f(); // 可以多次调用
}

fn call_fnonce<F: FnOnce()>(f: F) {
    f(); // 只能调用一次
}
