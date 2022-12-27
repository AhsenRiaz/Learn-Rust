use std::io;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Course(String, u32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct AlwaysEqual;

fn main() {
    learn_struct();

    //
    let _email = String::from("someone@gmail.com");
    let _username = String::from("someone");
    return_user(_email, _username);
    //
    calculate_area_of_rectangle()
}

fn learn_struct() {
    // if mutable we can update the struct otherwise not
    let mut user1 = User {
        email: String::from("someone@mail.com"),
        username: String::from("someuser"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // user1 moved to user2
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // user2 moved to user3
    let user3 = User {
        email: String::from("hey@mail.com"),
        ..user2
    };

    println!("{}", user3.active);
    //
}

fn return_user(email: String, username: String) -> User {
    User {
        active: false,
        username: username,
        email: email,
        sign_in_count: 4,
    }
}

fn tuple_struct() {
    let color = Color(0, 1, 67);
    let course = Course(String::from("English"), 1);
}

fn unit_struct() {
    // used as a marker
    let subject = AlwaysEqual;
}

fn calculate_area_of_rectangle() {
    println!("Enter the width");
    let mut _width = String::new();

    io::stdin()
        .read_line(&mut _width)
        .expect("Failed to read line");

    let _width = match _width.trim().parse::<u32>() {
        Err(_) => return,
        Ok(result) => result,
    };

    println!("Enter the height");
    let mut _height = String::new();
    io::stdin()
        .read_line(&mut _height)
        .expect("Enter the input");

    let _height = match _height.trim().parse::<u32>() {
        Ok(result) => result,
        Err(_) => return,
    };

    let rectangle = Rectangle {
        width: _width,
        height: _height,
    };

    println!("{:?}", rectangle);
    println!(
        "The area of width {} and height {} is {}",
        rectangle.width,
        rectangle.height,
        rectangle.area()
    );
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
