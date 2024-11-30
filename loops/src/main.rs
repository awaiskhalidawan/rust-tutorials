fn main() {
    println!("Printing array elements ...");

    let arr: [u32; 5] = [3, 3, 3, 3, 3];        // Create an array of 5 elements.

    for e in arr {                              // Print array.
        println!("{e} ");
    }
}
