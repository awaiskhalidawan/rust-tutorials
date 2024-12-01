fn main() {
    let s1 = String::from("Hello World!");              // Create a string on heap.
    let s2 = s1;                                        // Transfer the ownership of string from s1 to s2.
                                                        // s1 is not valid any more because the ownership is transferred.

    println!("s2 = {s2}");                              // Print s2.

    takes_ownsership_and_print(s2);                     // Transfer the ownership of s2 to function. 
                                                        // s2 is not valid any more because ownership is transferred.

    let s2 = String::from("Hello World!");              // Create a new string. We can reuse the variable name in Rust.
    let s3 = takes_ownsership_print_and_return(s2);     // Transfer the ownership of s2 to function and function will return back the ownership.
    println!("s5 = {s3}");
}

fn takes_ownsership_and_print(s3: String) {
    println!("s3 = {s3}");
}

fn takes_ownsership_print_and_return(s4: String) -> String {
    println!("s4 = {s4}");
    s4
}