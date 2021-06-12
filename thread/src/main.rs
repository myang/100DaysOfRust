use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("vector: {:?}", v);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("main {}", i);
        thread::sleep(Duration::from_millis(1));
    }

}
