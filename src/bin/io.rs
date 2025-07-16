use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let filename = "demo.txt";

    // 1. 创建并写入文件（如果文件存在会覆盖）
    let mut file = File::create(filename)?;
    file.write_all(b"Hello, Rust file operations!\n")?;
    println!("文件写入完成。");

    // 2. 读取文件内容
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("读取到的内容:\n{}", contents);

    // 3. 追加写入
    let mut file = OpenOptions::new().append(true).open(filename)?;
    let _ = file.write(b"by Rust file operations!\n");
    println!("追加写入完成。");

    // 再次读取文件验证
    let mut file = File::open(filename)?;
    contents.clear();
    file.read_to_string(&mut contents)?;
    println!("追加后的文件内容:\n{}", contents);

    Ok(())
}
