fn main() {
    let mut vec: Vec<i32> = Vec::new();                 // Create a vector.

    vec.push(1);                                        // Push the values to vector.
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);

    let third = &vec[2];                                // Create a immutable reference to third index of vector.

    println!("Third value of vector is {third}");
    println!("Size of the vector is {0}", vec.len());

    let mut vec1 = vec![1.0, 2.0, 3.0];                 // Create a vector using vec! macro.
    println!("Size of the vector is {0}", vec1.len());


    let idx = 1;
    let x = vec1.get(idx);                // Get the element from vec1 at index 1.

    match x {                                                 // Apply match expression on variable x.
        Some(e) => println!("Element at index: {idx} is {e}"),
        None => print!("No element found at index: {idx}")
    }

    for i in &vec1 {                                // Iterate over the vector using immutable reference.
        print!("{i} ");
    }

    println!();

    for i in &mut vec1 {                        // Iterate over the vector using mutable reference. Changing the value of the vector.
        *i = *i * 100.0;
        print!("{i} ");
    }

}
