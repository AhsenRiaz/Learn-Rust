fn main() {
    println!("Hello, world!");
    create_strings();
    update_string();
    concatenation();
    format_macro();
}

fn create_strings() {
    let s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    println!("string: {}", s);

    let s = String::from("hello");

    let hello = String::from("السلام عليكم");
    println!("{}", hello);
}

fn update_string() {
    let mut s = String::from("foo");

    s.push_str("bar");
    println!("{}", s);
}

fn concatenation() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = s1 + &s2;

    println!("{}", s3);
}

fn format_macro() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s);
}
