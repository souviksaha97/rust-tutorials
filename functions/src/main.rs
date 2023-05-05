fn main() {
    test();
    let z: u32 = add_numbers(10, 20);

    println!("{}", z);

    let number = {
        let x = 3;
        x + 1
    };
    println!("{}", number);
}

fn test() {
    println!("Test has been called...");
}

fn add_numbers(x: u32, y: u32) -> u32 {
    let result = x + y;
    if result > 10{
        return result - 10;
    };
    result
}
