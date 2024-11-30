fn main() {
    println!("Demonstration of tuples in Rust!");

    let tup: (u32, u32, f32) = (1000, 2000, 3000.3);              // Create a tuple of type (u32, u32, f32)

    let (x,y,z) = tup;                                            // Access the tuple.

    println!("The value of tuple (x, y, z) = ({x}, {y}, {z})");   // Print the tuple values. (1st way)

    println!("The value of tuple (x, y, z) = ({}, {}, {})", tup.0, tup.1, tup.2);   // Print the tuple values. (2nd way)
}
