use rand::Rng;  

fn main() {
    println!("Generating a random number ...");             
    let number = rand::thread_rng().gen_range(1..=1000);    // Using rand crate to generate a random number.
    println!("The generated random number is: {number}");   // Printing the generated random number on screen.
}
