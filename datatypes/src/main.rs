fn main() {
    let x: i32 = 2;
    let y: f32 = 2.401;
    let bool_var: bool = false;
    let letter: char = 'a';

    println!("{} {} {} {}", x, y, bool_var, letter);


    let mut tup = (1, true, 's');
    println!("{}", tup.1);
    tup.1 = false;

    println!("{}", tup.1);
    

    let mut arr: [i32; 5] = [1,2,3,4,5];
    println!("{}", arr[2]);

    arr[2] = 10;
    println!("{}", arr[2]);

    println!("{:?}", tup);
}
