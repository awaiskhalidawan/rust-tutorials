#[derive(Debug)]
enum Taste {                // Create an enumerator 'Taste'.
    Sweet,
    Citrus,
    Bitter
}

#[derive(Debug)]
enum Fruit {                // Create an enumerator 'Fruit'.
    Apple(Taste),           // The enumerator value 'Apple' has an additional property 'Taste'.
    Orange,
    Mango,
    Strawberry,
    Blueberry,
    Apricot,
    Pineapple
}

fn main() {
    let fruit: Fruit = Fruit::Apple(Taste::Sweet);          // Create a variable 'fruit' with 'Apple: Sweet'.

    match fruit {                                           // Code flow using match expression.
        Fruit::Apple(taste) => println!("An Apple with {taste:?} taste"),
        Fruit::Orange => println!("An Orange"),
        Fruit::Mango => println!("A Mango"),
        Fruit::Blueberry => println!("A Blueberry"),
        other => println!("Unknown fruit"),
        _ => todo!()
    }

    let mut x: Option<u32> = Some(10001);        // Create an optional u32 variable.

    match x {                                    // Code flow using match expression.
        Some(1000) => println!("variable x contains value 1000"),
        Some(num) => println!("variable x contains value {num}"),
        _ => ()
    }

    if let Some(y) = x {                         // Pattern matching using if let statement.
        println!("variable x contains value {y} (Pattern matching using if let statement)");
    }
}
