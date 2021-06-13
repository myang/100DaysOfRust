use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let v = vec![1, 2, 3];
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        for _ in 1..10 {
            println!("vector: {:?}", v);
            thread::sleep(Duration::from_millis(1));
        }

        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("main {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

}
