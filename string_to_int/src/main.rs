use std::io;
use std::cmp::Ordering;
use std::process::exit;

fn main() {
    println!("Enter input: ");

    let mut str_number = String::new();           // Create an empty string.
    io::stdin()
        .read_line(&mut str_number)
        .expect("Unable to get input string.");   // Take an input from user.


    let number: u32 = match str_number.trim().parse() {  // Parse the input string.
        Ok(num) => num,                                  // Set the value if the parsing is successful.
        Err(_) => exit(1),                               // Exit the process with return code 1 if parsing is not successful.
    };

    println!("Input string: {str_number}");         // Print the input string.
    println!("Input string (parsed): {number}");    // Print the parsed integer value.

    let temp: u32 = 44;                             // Declare a number with value 44.
    match number.cmp(&temp) {                       // Compare the parser number with declared number and
        Ordering::Less => {                         // print the result accordingly.
            println!("Less");
        }
        Ordering::Equal => {
            println!("Equal");
        }
        Ordering::Greater => {
            println!("Greater");
        }
    }
}
