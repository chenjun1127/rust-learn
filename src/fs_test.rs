// use std::fs;

 


use std::{fs, io::Write};

pub(crate) fn fs_test() {
    let mut text = fs::read("/Users/chenjun/rust/rust-demo/src/text.txt").expect("read file error");
    let mut vec2 = vec![4, 5, 6];
    text.append(&mut vec2);
    text.write(b"11111").unwrap();
    println!("{:?}", text);
}
