use std::io::stdin;

pub fn io_test() {
    let args = std::env::args();
    println!("{:?}", args);
}

pub fn io_test_2() {
    let mut str_buf = String::new();
    stdin()
        .read_line(&mut str_buf)
        .expect("Failed to read line");
    println!("Your input line is \n{}", str_buf);
}
