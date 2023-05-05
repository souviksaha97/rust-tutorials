use std::io;


fn main() {
    // Read input from the console
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Split input into individual numbers
    let numbers = input
                                                        .split_whitespace()
                                                        .map(|s| s.parse::<i32>()
                                                        .unwrap());

    // Find the smallest number
    let smallest = numbers
                        .min()
                        .unwrap();

    // Print the smallest number
    println!("The smallest number is {}", smallest);
}
