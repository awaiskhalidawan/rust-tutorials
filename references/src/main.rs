fn main() {
    let mut s1 = String::from("Hello World!");  // Create a mutable string.

    let s2 = &s1;                               // Create a immutable reference to string s1.

    println!("s1 = {s1}, s2 = {s2}");           // Print the string.

    let s3 = &mut s1;                           // Create a mutable reference. Now the immutable reference s2 
                                                // is invalidated here. It cannot be used any further.

    println!("s3 = {s3}");                      // Print the string using the mutable reference.

    s3.push_str(" +++ ");                       // Update the string using mutable reference.

    println!("s1 = {s1}");                      // Print the string using the actual variable. Now the mutable
                                                // reference is invalidated. We cannot use it now.
    
    function1(&s1);                             // Pass the actual string to function which takes a immutable 
                                                // reference.

    println!("s1 = {s1}");                      // Print the string.
}

fn function1(str: &String) {
    println!("str = {str}");
}