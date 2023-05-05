use std::io;

fn main() {
    let x: u8 = 12;
    let y: u8 = 10;

    let z = x + y;
    println!("{}", z);

    let z = x - y;
    println!("{}", z);

    let z = x * y;
    println!("{}", z);

    let z = x / y;
    println!("{}", z);

    let z = x % y;
    println!("{}", z);

    let x = 12700 as i64;
    let y = 10 as i32;

    let z = x / y as i64;
    println!("{}", z);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");

    let int_input: i64 = input.trim().parse().unwrap();

    println!("{}", int_input+2);
}
