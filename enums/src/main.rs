enum IpAddrKind {
    V4,
    V6,
}

struct IpAddrV10 {}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum IpAddr {
    V4(String),
    V6(String),
    V2(u8, u8, u8, u8),
    V10(IpAddrV10),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// <T> means the Some variant of the Option enum can hold one piece of data of any type, and that each concrete type that gets used in place of T makes the overall Option<T> type a different type.
// enum Option<T> {
//     None,
//     Some(T),
// }

impl Message {
    fn call(&self) {}
}

fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route();

    value_in_cents(Coin::Penny);

    let five = Some(5);
    let six = plus_one(five);
    println!("six: {:?}", six);
    let none = plus_one(None);
    println!("none: {:?}", none);

}

fn attach_data_to_enum() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    let v2 = IpAddr::V2(127, 0, 0, 1);
}

fn route() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

fn enum_of_generic_type() {
    let some_number = Some(10);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
