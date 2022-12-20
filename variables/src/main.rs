// fn main() {
//     let mut x = 5;
//     println!("x is {}", x);

//     x = 10;
//     println!("x is {}", x);
// }

// fn main() {
//     // shadowing
//     // shadowing only occurs with immutable variables
//     let x = 3;
//     let x = x + 3; // 6
//     let x = x * 2; // 12

//     println!("The value of x is : {}", x);
// }

// fn main() {
//     //floating integers have two types f32 and f64
//     // by default type is f64
//     let a = 2.0;
//     let b = 3.0;
//     let z = a + b;
// }

// Compound types ( tuples and arrays )

// fn main() {
//     let tup = ("ahsen", 4.7, 100);
//     let (a, b, c) = tup;
//     println!("value of a is : {}, b is: {} and c is {}", a, b, c);

//     let x = tup.0;
//     let y = tup.1;
//     let z = tup.2;
//     println!("value of x is : {}, y is: {} and z is {}", x, y, z);
// }

fn main() {
    let names = ["john", "doe", "jack", "jill"];

    let a = [5, 1, 100];
    a[0];
    a[1];

    let b = [3; 5]; // [3,3,3,3,3] [value; numberOfElements];
}
