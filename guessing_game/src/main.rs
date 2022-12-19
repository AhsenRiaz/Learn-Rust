// use - a keyword used to import modules
// std - standard library (set of basic modules)
// io - an input output module contains helpers for input output funcionality

// a module for random number generation
use rand::Rng;
// a module for comparint from which import Ordering enum
use std::cmp::Ordering;
// a module for handling user inputs
use std::io;

// main function - an entry point to the program
fn main() {
    // prints with a new line
    println!("Guess the number");

    println!("Please input your guess");

    let secret_number = rand::thread_rng().gen_range(0..101);
    println!("The secret number is: {}", secret_number);

    // mut - clarify that the variable is mutable - it can be updated.
    // String - String type provided by rust standard library
    // new(): a function that creates a new empty string
    // :: - indicated that new() functions is associated to the String type
    // summarize:  create a mutable variable named guess that is bound to a new, empty instance of a String

    // an infinite loop
    loop {
        let mut guess = String::new();

        // stdin() is associated to io module. It returns a type Stdin which is a  handle to to the standard input for your terminal
        // read_line() - use to take user input
        // takes an argument string. The string needs to be mutable so the method can change the content of the variable.
        // & - represents that the argument passed is a reference. meaning that the code is not creating a copy of the guess variable in the memory. Insead it is calling the before created reference of the
        // expect - read_line returns Result type. It can be Ok or Err. If err it is caught by expect function. If Result is OK expect will just pass along the Ok value
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            // _ to ingore the argument field
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        // match is used for creating a conrtol flow base on pattern matching
        match guess.cmp(&secret_number) {
            // An arm
            // left side is the pattern
            // right side is the execution
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // to break the loop
                break;
            }
        }
    }
}
