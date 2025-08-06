use std::sync::{Arc, Mutex};
// Mutex::new(0)：创建一个初始值为 0 的互斥锁。
// Arc::new(...)：将互斥锁包裹进 Arc，以便多个线程共享。
// Arc::clone(&counter)：每个线程都拿到同一个数据的“共享所有权”。
// thread::spawn(move || { ... })：开启新线程，使用 move 捕获变量。
// counter.lock().unwrap()：
// lock() 返回一个 Result<MutexGuard<_>>，这里使用 unwrap() 解包。
// 拿到锁后，返回一个 MutexGuard，可用来访问/修改数据。
// *num += 1：对内部数据加 1。
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
    println!("All threads have finished execution.");
}
