use std::thread;

fn main() {

    // Create a new thread which prints the value of i from 1 to 10.
    let handle = std::thread::spawn(|| {
        for i in 1..10 {
            println!("Thread 1: i = {}", i);
            thread::sleep(std::time::Duration::from_millis(200));
        }
    });

    // Wait for the thread to finish.    
    handle.join().unwrap();

    // Main thread prints the value of i from 1 to 5.
    for i in 1..5 {
        println!("Main Thread: i = {}", i);
        thread::sleep(std::time::Duration::from_millis(200));
    }

    // Create a new thread which moves a variable from main thread to child thread and increments it by 3.
    let mut counter: u16 = 0;
    let handle = std::thread::spawn(move || {
        counter += 3;
        println!("Child thread: counter = {}", counter);
    });

    // Main thread increments the counter by 1. This counter is different from the counter in the child thread.
    counter += 1;
    println!("Main thread: counter = {}", counter);

    // Wait for the thread to finish.
    handle.join().unwrap();
}
