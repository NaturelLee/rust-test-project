extern crate rand;

use rand::{thread_rng, Rng};

pub fn run() {
    let i: i32 = rand::random();
    println!("the random i32 is {}", i);

    let x: u8 = rand::random();
    println!("u8 random is {}", x);

    let x: f64 = rand::random();
    println!("f64 random is {}", x);

    let x: bool = rand::random();
    println!("the random bool is {}", x);

    let mut rng = thread_rng();
    let y: f64 = rng.gen_range(-10.0, 10.0);
    println!("number from -10.0 to 10.0 is {}", y);
    println!("number from 0 to 9 is {}", rng.gen_range(0, 10));

    for i in 1..10 {
        println!("random number #{}: {}", i, rng.gen_range(0, 100));
    }

    let mut arr = [0i32, 9];
    thread_rng().try_fill(&mut arr[..]);
    println!("random number array {:?}", arr);

    let distr = rand::distributions::Uniform::new_inclusive(1, 100);
    let mut nums = [0i32, 3];

    for x in &mut nums {
        *x = rng.sample(distr);
    }

    println!("some numbers: {:?}", nums);
}
