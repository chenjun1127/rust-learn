fn main() {
    println!("My name is {}", "justin");
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    //命名参数
    println!("{} is {age} years old", "Alice", age = 30);
    println!("{} of {:b} people know binary, the other half don't", 1, 2);
    //格式化数字

    println!("This struct contains: {:?}", Strc(42));
    let name = String::from("Bob");
    let p = Persion {
        name, // 发生所有权移动
        age: 25,
    };
    println!("{:#?}", p);
    println!("{} is {} years old", p.name, p.age);
    // println!("name ,{}",name); //name 变量已经发生所有权移动，无法再使用
    let a = Animal {
        name: "dog",
        age: 3,
    };
    println!("{:?}", a);
    let arr = [1, 2, 3];
    println!("Array: {:?}", arr);
    let slice = &arr[1..3];
    println!("Slice: {:?}", slice);
}
#[allow(dead_code)]
#[derive(Debug)]
struct Strc(i32);

#[derive(Debug)]
#[allow(dead_code)]
struct Persion {
    name: String,
    age: u8,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Animal<'a> {
    name: &'a str,
    age: u8,
}
