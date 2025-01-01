use std::rc::Rc;
use std::cell::RefCell;

struct Data {
    x: i32,
    y: i32,
}

impl Drop for Data {
    fn drop(&mut self) {
        println!("Dropping Data with x = {}, y = {}", self.x, self.y);
    }
}

fn main() {
    // Demonstrate the use of Box pointer.
    let x = Box::new(Data { x: 10, y: 20 });            // Create a structure on heap memory.
    println!("x.x = {}, x.y = {}", x.x, x.y);           // Print the values of x.x and x.y.

    let mut y = x;                                      // Transfer the ownership of x to y.
    println!("y.x = {}, y.y = {}", y.x, y.y);           // Print the values of y.x and y.y.
    std::mem::drop(y);                                  // Drop the structure y using std::mem::drop. It will drop the memory before y goes out of scope.

    // Demonstrate the use of Rc (Reference Counted) pointer.
    let z = Rc::new(Data { x: 30, y: 40 });             // Create a reference counted structure on heap memory.
    println!("z.x = {}, z.y = {}", z.x, z.y);           // Print the values of z.x and z.y.

    let w = Rc::clone(&z);                              // Clone the reference counted structure z.
    println!("w.x = {}, w.y = {}", w.x, w.y);           // Print the values of w.x and w.y.
    println!("Reference count of z = {}", Rc::strong_count(&z)); // Print the reference count of z.
    std::mem::drop(w);                                  // Drop the reference counted structure w.
    println!("Reference count of z = {}", Rc::strong_count(&z)); // Print the reference count of z.

    // Demonstrate the use of Weak pointer.
    let x = Rc::new(Data { x: 30, y: 40 });             // Create a reference counted structure on heap memory.
    let y = Rc::downgrade(&x);                          // Create a weak pointer from the reference counted structure x.
    println!("Reference count of x = {}", Rc::strong_count(&x)); // Print the reference count of x.
    println!("Reference count of y = {}", Rc::weak_count(&x));   // Print the weak reference count of x.
    let z = y.upgrade();                                // Upgrade the weak pointer to reference counted pointer.

    // Demonstrate the use of RefCell pointer.
    let x = RefCell::new(Data { x: 50, y: 60 });        // Create a RefCell structure on heap memory.
    println!("x.x = {}, x.y = {}", x.borrow().x, x.borrow().y); // Print the values of x.x and x.y.

    let w = x.borrow_mut();                             // Get the mutable reference of x using borrow_mut. 

    // A RefCell allows one mutable reference or multiple immutable references at a time. The condition are checked at runtime.
    // The following code will panic at runtime because we are trying to get the immutable reference of x using borrow while there 
    // is already a mutable reference of x taken already.
    let y = x.borrow();                                 // Runtime Error: Get the immutable reference of x using borrow.
    let z = x.borrow();                                 // Runtime Error: Get the immutable reference of x using borrow.
}

