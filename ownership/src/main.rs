fn main() {
  

    slice_it(); // word will get the value 5

    let a_string = String::from("hello");

    // pass the reference of a_string
    let length = calculate_length(&a_string);
    println!("The length of a_string: {} is: {}", a_string, length);

    let mut b_string = String::from("stack");
    modify_mutable_reference(&mut b_string);

    return_ownership_concept();
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's values move into the function || transfer ownership
                        // s's value no longer valid

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function, copy of it would be created
                   // x still valid for use
} // x goes out of scope so drop function called on it

fn takes_ownership(some_string: String) {
    println!("takes_ownership: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("some_integer: {}", some_integer);
}

fn return_ownership_concept() {
    let s1 = gives_ownership(); // dropped

    let s2 = String::from("hello"); // invalidated

    let s3 = gives_and_takes_back(s2); // dropped
}

fn gives_ownership() -> String {
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // moves its ownership to s1
}

fn gives_and_takes_back(a_string: String) -> String {
    a_string
}

// if we want to pass a variable without transferring the ownership, we use references
fn calculate_length(s: &String) -> usize {
    // because we are borrowing the reference from the owner we can modity it
    // compiler will give error
    // the only way we can modify a borrowed reference is that owner is mutable;
    // s.push_str(", xworld");

    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn modify_mutable_reference(s: &mut String) {
    s.push_str(",overflow");
}

// NOTE : RUST DOES NOT ALLOW A DANGLING REFERENCE
// A LINK OR POINTER TO AN INSTRUCTION THAT DOESNT EXIST ANYMORE
// WILL GIVE AN ERROR DURING COMPILE TIME

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

// The Rules of References
// Let’s recap what we’ve discussed about references:

// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.

// THE SLICE TYPE

fn slice_it()  {
   let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} and {}", hello, world);

}
