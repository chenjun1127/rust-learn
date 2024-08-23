use std::io::BufWriter;

use ferris_says::say;

use fs_test::fs_test;
use io::io_test;
use io::io_test_2;
use struct_test::MyStruct;
use crate::calc::test_area;

mod io;
mod struct_test;

mod fs_test;

mod vector_test;
mod calc;

fn main() {
    println!("Hello, world!");
    test();
    slice();
    main_test();
    let m = MyStruct::new(String::from("jack"), 22);
    m.some_function();
    io_test();
    io_test_2();
    fs_test();
    test_area();
}

fn test() {
    let stdout = std::io::stdout();
    let msg = String::from("hello world");
    let width = msg.chars().count();
    let mut writer = BufWriter::new(stdout.lock());

    say(msg.as_bytes(), width, &mut writer).unwrap();
    let remainder = 43 % 5; // 求余
    print!("x 的 fwhg 为{}", remainder);
    let arr = [5, 34, 4, 1];
    for i in arr.iter() {
        println!("--------{}, ", i);
    }

    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    loop {
        let ch = s[i];
        if ch == 'O' {
            break;
        }
        println!("\'{}\'", ch);
        i += 1;
    }
}

fn slice() {
    let mut s = String::from("hello world");
    let part1 = &s[..5];
    let part2 = &s[6..11];
    println!("{}---{}", part1, part2);
    s.push_str("！！！！");
    println!("{}", s);
}

fn main_test() {
    let info = Info {
        name: String::from("jack"),
        age: 22,
    };
    println!("age:{},name:{}", info.age, info.name);
}

struct Info {
    name: String,
    age: u32,
}
