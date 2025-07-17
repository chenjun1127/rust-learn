fn main() {
    let color = String::from("green");
    let print = || println!("color is {}", color);
    print();
    let _color = &color;
    print();
    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("{}", count);
    };
    inc();
    // let _reborrow = &mut count;
    // println!("{}", _reborrow);
    inc();
    let _reborrow = &mut count;
    println!("{}", _reborrow);
    // inc(); // ❌ 闭包想要再次借用 count 的可变引用，但前面的引用还在
}
