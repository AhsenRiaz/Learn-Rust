use std::{fs::File, io::ErrorKind};

fn main() {
    open_file()
}

fn open_file() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

// we can also use expect() and unwrap("") for the error handling.
// we can also use ? operator. It works like match but what it returns is defined at the function
