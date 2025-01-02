mod utils;                          // Declare utils.rs as a module.

use utils::increament;              // Use utils::inreament function from utils module.

fn main() {
    let mut x: i32 = 100;           // Create a mutable variable x.

    x = increament(x);              // Call the increament function and pass x to it and read the return value.

    println!("Value of x is {x}");  // Print the value of x.
}
