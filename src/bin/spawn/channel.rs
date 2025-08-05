use std::{thread, time::Duration};
fn main() {
    let (tx, rx) = std::sync::mpsc::channel();
    let s = "Hello, world!";
    thread::spawn(move || {
        for i in 0..5 {
            println!("spawned: {} {}", i, s);
            thread::sleep(Duration::from_millis(1));
            tx.send(i).unwrap();
        }
    });
    // while let Ok(value) = rx.recv() {
    //     println!("Received from spawned thread: {}", value);
    // }
    for received in rx.iter() {
        println!("Received from spawned thread: {}", received);
    }
}
