use std::io;

fn main() {
    println!("Enter input: ");
    
    let mut str_number = String::new();           // Create an empty string.
    io::stdin()
        .read_line(&mut str_number)
        .expect("Unable to get input string.");   // Take an input from user.

    let number: u32 = str_number.trim().parse().expect("Input string is not a number.");   // Parse the input string.

    println!("Input string: {str_number}");         // Print the input string.
    println!("Input string (parsed): {number}");    // Print the parsed integer value.
}
