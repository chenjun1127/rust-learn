use std::{thread, time::Duration};

fn spawn_function() {
    for i in 0..5 {
        println!("spawned: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn main() {
    let handle = thread::spawn(spawn_function);
    for i in 0..5 {
        println!("main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}
