use std::collections::HashMap;

fn main() {
    create_hashmap();
    access_hashmap();
    hashmap_ownership_concept();
    insert_after_checking();
}

fn create_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);
}

fn access_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score: {}", score);

    let team_name = String::from("Red");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("score: {}", score);

    for (key, value) in &scores {
        println!("{key}, {value}");
    }
}

fn hashmap_ownership_concept() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    for (key, value) in &map {
        println!("{key}, {value}");
    }
}

fn insert_after_checking() {
    // Using the entry method to only insert if the key does not already have a value

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}
