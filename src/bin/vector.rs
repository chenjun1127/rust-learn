fn main() {
    let mut a: Vec<char> = vec!['a', 'b'];
    println!("Vector a: {:?}", a);
    let mut b: Vec<char> = vec!['c', 'd', 'e'];
    println!("Vector b: {:?}", b);
    a.append(&mut b);
    println!("Vector a: {:?}", a);
    println!("Vector b: {:?}", b);
    let mut str1 = vec![String::from("Hello"), String::from("World")];
    let str2 = vec![String::from("Foo"), String::from("Bar")];

    str1.extend(str2);

    println!("Vector str1: {:?}", str1);

    // b 不能再用了，因为所有权被转移了
}
