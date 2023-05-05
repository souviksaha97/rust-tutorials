fn main() {
    let x = 4;
    println!("x is : {}", x);

    let mut y = 10;
    println!("y is : {}", y);

    y = 2;
    println!("y is : {}", y);

    let y = y+1;
    println!("y is : {}", y);

    let x = "hello";
    println!("x is : {}", x);

    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{}", SECONDS_IN_MINUTE);
}
