use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();                        // Create a new hashmap.

    scores.insert(String::from("Blue"), 10);                // Insert key/value pair in hashmap.
    scores.insert(String::from("Yellow"), 50);              // Insert key/value pair in hashmap.

    let val = scores.get("Blue").copied().unwrap_or(0);     // Get the value for key "Blue" from hashmap.
    println!("val for Blue = {val}");                       // Print the value.

    let val = scores.get("Red").copied().unwrap_or(0);      // Get the value for key "Red" from hashmap.    
    println!("val for Red = {val}");                        // Print the value. The value will be 0 because key does not exist.

    scores.entry(String::from("Red")).or_insert(110);       // Insert key/value pair in hashmap if key does not exist.

    println!("Printing hashmap key/values ... ");           // Print all key/values in hashmap.
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
