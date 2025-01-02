fn main() {
    let s1: String = String::from("Hello world!");     // Create a new string s1.

    let s2 = &s1[0..5];   // Take a 'slice' from the string s1 of first five bytes.

    let s3 = &s1[3..7];   // Take a 'slice' from the string s1 from byte position 3 upto (7 - 3) = 4 bytes.

    println!("s1 = {s1}");
    println!("s2 = {s2}");
    println!("s3 = {s3}");
}
