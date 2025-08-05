// 生命周期不是你用来“控制生命周期”的，而是告诉编译器引用活多久。
fn main() {
    let s = String::from("Hello, Rust!");
    let r = get_ref(&s);
    println!("r = {}", r);
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let longer = logerest(&s1, &s2);
    println!("The longer string is: {}", longer);
}

fn get_ref<'a>(s: &'a String) -> &'a String {
    s
}

fn logerest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
