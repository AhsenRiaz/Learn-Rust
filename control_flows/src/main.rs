use std::io;
fn main() {
    // let mut input = String::new();
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Failed to read line");
    // let input = input.trim().parse::<u32>().expect("provide a valid number");
    // println!("Is x greater than 5: {}", check(input));

    // let number = -5;
    // let is_positive = if number >= 5 { true } else { false };
    // println!("the value of is_positive is: {}", is_positive);

    divisible_by(10);

    count_to_ten();

    countdown_from(10);

    loop_numbers();

    countdown_to_five();

    countdown_from_five();
}

fn check(x: u32) -> bool {
    if x > 5 {
        true
    } else {
        false
    }
}

fn divisible_by(number: u32) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 5 == 0 {
        println!("number is divisible by 5");
    } else if number % 2 == 0 {
        println!("number is divisible by 2")
    } else {
        println!("number is not divisible by 2,4,5");
    }
}

fn count_to_ten() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // returns 20 wheater semicolon present or not due to break keyword
        }
    };

    println!("the counter is: {}", result);
}

fn countdown_from(x: u32) {
    let mut countdown = x;

    while countdown != 0 {
        println!("{}!", countdown);
        countdown -= 1
    }
}

// somehow not a good practive in rust. This code is slow cause it needs to check everytime.
fn loop_numbers() {
    let a = [1, 2, 3, 4, 5];

    let mut index = 0;

    while index < 5 {
        println!("the value is: {} at index: {}", a[index], index);
        index += 1;
    }
}


// this code is more efficient and safe as will not result of going out of bounds
fn countdown_to_five() {
    let a = [1, 2, 3, 4, 5];

    for element in a.iter() {
        println!("{}", element);
    }
}

fn countdown_from_five(){
    let a = (1..=6).rev(); // includes 6
    for element in a {
        println!("element: {}", element);
    }

    for number in (1..7).rev() {
        println!("number: {}", number);

    }
}
