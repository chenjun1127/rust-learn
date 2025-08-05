use std::fs::File;
use std::io;
use std::io::Read;
fn main() {
    let f = File::open("non_existent_file.txt");
    // match f {
    //     Ok(file) => println!("File opened successfully: {:?}", file),
    //     Err(e) => eprintln!("Error opening file: {}", e),
    // }
    if let Ok(file) = f {
        println!("File opened successfully: {:?}", file);
    } else if let Err(e) = f {
        eprintln!("Error opening file: {}", e);
    } else {
        eprintln!("An unexpected error occurred.");
    }
    // File::open("hello.txt").unwrap();
    println!("Hello, world!");
    let file = read_file("hello.txt");
    match file {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                eprintln!("File not found: {}", e);
            }
            std::io::ErrorKind::PermissionDenied => {
                eprintln!("Permission denied: {}", e);
            }
            _ => {
                eprintln!("An unexpected error occurred: {}", e);
            }
        },
    }
}
fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
