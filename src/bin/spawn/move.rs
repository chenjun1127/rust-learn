use std::{thread, time::Duration};

fn main() {
    let s = "Hello, world!";
    let handle = thread::spawn(move || {
        for i in 0..5 {
            println!("spawned: {} {}", i, s);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
}
