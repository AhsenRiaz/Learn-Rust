fn main() {
    let y = -5;
    another_function(10, 20);
    let y = expression_example();
    println!("the returned value is : {}", y);

    add_three(y);
}

fn another_function(x: u32, y: u32) {
    let z = x + y;

    println!("The result of x:{} and y:{} is z:{}", x, y, z);
}

fn expression_example() -> i32 {
    // statement perform some action but does'nt return anyting
    // expressions return some value

    let x = 5;
    // no semicolon means that its an expression and expression return a value
    x + 1
}

fn add_three(y: i32) -> i32 {
    y + 3
}
