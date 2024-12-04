struct User {
    active: bool,
    name: String,
    email: String
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
}

fn main() {
    let user1 = User {
        active: true,
        name: String::from("Muhammad Awais Khalid"),
        email: String::from("awais.khalid.awan@gmail.com")
    };

    println!("user1 = [{0}, {1}, {2}]", user1.active, user1.name, user1.email);    

    let user2 = User {              // Some of the values (name, email) of user1 is moved to user2. user1 is
        active: true,               // not valid any more.
        ..user1
    };

    //println!("user1 = {0}", user1.name);    // This statement will create a compilation error.
    println!("user2 = [{0}, {1}, {2}]", user2.active, user2.name, user2.email);

    let rec = Rectangle {           // Create a rectangle struct.
        width: 30,
        height: 50
    };

    let area1 = calculate_area(&rec);        // Calculate the rectangle area using the function.
    let area2 = rec.area();                 // Calculate the rectangle area using the associated struct function.

    println!("{rec:?}   Area = {area1}   Area = {area2}");    // Print the rectangle and its area.
}

fn calculate_area(rec: &Rectangle) -> u32 {
    return rec.width * rec.height;
}