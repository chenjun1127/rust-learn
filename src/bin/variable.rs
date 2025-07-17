//变量
fn main() {
    //不可变变量
    let x = 5;
    println!("The value of x is: {}", x);
    //x = 6; //error: cannot assign twice to immutable variable `x`

    //可变变量
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);
    let long_lived_binding;
    {
        let short_lived_binding = 5;
        println!("The value of short_lived_binding is: {}", short_lived_binding);
        long_lived_binding = short_lived_binding;
        println!("The value of long_lived_binding is: {}", long_lived_binding);
    }
    //println!("The value of short_lived_binding is: {}", short_lived_binding); //error: use of moved value: `short_lived_binding`
    let long_lived_binding = 10;
    println!("The value of long_lived_binding is: {}", long_lived_binding);
}
