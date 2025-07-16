pub fn print_result(res: Result<u32, &str>) {
    match res {
        Ok(n) => println!("Success: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
pub fn print_option(opt: Option<&str>) {
    match opt {
        Some(val) => println!("Got: {}", val),
        None => println!("No value"),
    }
}