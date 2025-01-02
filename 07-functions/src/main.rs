fn main() {
    println!("Hello, world!");
    function1();                                                // Called 1st function.
    function2(10);                                              // Called 2nd function.
    let x = function3(10);                                      // Called 3rd function.
    println!("The return value of function3 is {x}");
    let x = function4(20, 30);                                  // Called 4th function.
    println!("The return value of function4 is {x}");
}

fn function1() {                                                // Created a 1st function.
    println!("function1 called ... ");
}

fn function2(x: u32) {                                          // Created a 2nd function with input parameter x.
    println!("function2 called with param x = {x}");
}

fn function3(x: u32) -> u32 {                                   // Created a 3rd function with input parameter x.
    x + 1
}

fn function4(x: u32, y: u32) -> u32 {                           // Created a 4th function with input parameter x.
    x + y
}