enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    println!("Hello, world!");
    create_vector();
    updating_vector();
    read_vector();
    iterate_vec();
    iterate_mut_vec();
    enum_vec();
}

fn create_vector() {
    let v: Vec<i32> = Vec::new();

    let v_macro = vec![1, 2, 3];
}

fn updating_vector() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
}

fn read_vector() {
    // indexing syntax method
    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("The third element is : {}", third);

    // get method
    let third = v.get(3);
    match third {
        Some(third) => println!("The third element is: {}", third),
        None => println!("There is no third element"),
    }
}

fn iterate_vec() {
    let v = vec![2, 4, 6, 8];

    for i in &v {
        println!("{i}");
    }
}

fn iterate_mut_vec() {
    let mut v = vec![3, 5, 7];

    for i in &mut v {
        *i += 50;
    }
}

fn enum_vec() {
    let row = vec![
        SpreadsheetCell::Int(34),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
