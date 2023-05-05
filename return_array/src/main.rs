extern crate rand;
// import commonly used items from the prelude:
use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();
    println!("Hello, world!");
    let mut arr1: [i32; 5]  = [0i32; 5];
    let mut arr2: [i32; 5]  = [0i32; 5];

    let distr = rand::distributions::Uniform::new_inclusive(1, 100);

    for x in &mut arr1 {
        *x = rng.sample(distr);
    }

    for x in &mut arr2 {
        *x = rng.sample(distr);
    }

    let sum: [i32; 5] = add_array(arr1, arr2);

    println!("{:?} + {:?} = {:?}", arr1, arr2, sum);
}


fn add_array(a: [i32; 5], b: [i32; 5]) -> [i32; 5] {
    let mut arr: [i32; 5] = [0; 5];
    for i in 0..5 {
        arr[i] = a[i] + b[i];
    }
    arr
}