fn main() {
    another_function(5);

    let x = plus_one(5);
    println!("{}", x);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
