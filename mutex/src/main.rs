use std::thread;
use std::sync::Mutex;
use std::sync::Arc;

fn main() {

    // Create a mutex over an integer. The Mutex is wrapped in an Arc to allow it to be shared between threads.
    let mtx = Arc::new(Mutex::new(0));
    
    // Clone the Arc so that it can be moved into the thread.
    let mtx1 = mtx.clone();

    // Spawn a thread that will lock the mutex and increment the value by 10.
    let th1 = thread::spawn(move || {
        println!("Chile thread started ... ");
        let mut data = mtx1.lock().unwrap();
        println!("Chile thread: data = {}. Increamenting it by 10 ... ", *data);
        *data += 10;
    });

    // Wait for the thread to finish.
    th1.join().unwrap();
    println!("Main thread: child thread has finished.");

    // Lock the mutex and print the value.
    let data = mtx.lock().unwrap();
    println!("Main thread: data = {}", *data);
}
