fn main() {
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // let s3 = s2;
    // println!("{}, world!", s3);
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    println!("------------------");
    let s = String::from("Hello, world!");
    println!("{}", s);
    take_ownership(s);
    // println!("{}", s); // This line would cause an error because `s` has been moved.
    let is_new = true;
    println!("Is it new? {}", is_new);
    makes_copy(is_new);
    println!("Is it new? {}", is_new); // This is fine because `is_new` is a boolean and implements the Copy trait.\
    println!("------------------");
    let mut s = String::from("hello");

    change(&mut s);
    println!("{}", s); // This will print "hello, world"
}
fn calculate_length(s: &str) -> usize {
    s.len()
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_bool: bool) {
    println!("{}", some_bool);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
/*
常见 Copy 类型（栈上固定大小的数据）
所有整数类型（i32, u64…）
所有浮点数类型（f32, f64）
bool
char
固定大小的元组（比如 (i32, i32)），前提是里面的元素也都是 Copy 类型
注意：&T（不可变引用）也是 Copy

常见非 Copy 类型（堆分配或动态大小的数据）
String
Vec<T>
HashMap<K,V>
Box<T>
以及包含了以上类型的元组或结构体
 */
