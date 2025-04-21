
// A function which takes two arguments and return and optional result.
fn add(x: u32, y: u32) -> Option<u32> {
    if x > 10 && y > 10 {
        // Return the result of addition if x and y are greater than 10.
        return Some(x + y)
    }

    // Return 'none' if x and y are less than 10.
    None
}

fn main() {
    // Declare two immutables.
    let x: u32 = 20;
    let y: u32 = 20;

    // Call the 'add' function.    
    let result = add(x, y);

    // Check if the function has returned some value.
    if result.is_some() {
        // Function returned some value.
        println!("Addition result = {}", result.unwrap());  
    } else {
        // Function didn't returned a value.
        println!("No result returned from function add(). ");
    }
}

