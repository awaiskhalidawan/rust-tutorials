use std::sync::mpsc;
use std::thread;

fn main() {
    // Create a new interthread communication channel.
    let (tx, rx) = mpsc::channel();

    // Clone the sender tx to create another sender tx2.
    let tx2 = tx.clone();

    // Create a thread which sends and integer value to the main thread.
    let th1 = std::thread::spawn(move || {
        for i in 1..10 {
            tx.send(i).unwrap();
            thread::sleep(std::time::Duration::from_millis(200));
        }
    });

    // Create another thread which sends and integer value to the main thread.
    let th2 = std::thread::spawn(move || {
        for i in 1..10 {
            tx2.send(i).unwrap();
            thread::sleep(std::time::Duration::from_millis(200));
        }
    });

    // Wait for both threads to complete.
    th1.join().unwrap();
    th2.join().unwrap();

    // Main thread receives the integer value from the child thread via the channel.
    for i in rx {
        println!("Main Thread Receives: i = {}", i);
    }
}
