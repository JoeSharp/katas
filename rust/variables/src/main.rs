const ONE_DAY_IN_SECONDS: u32 = 60 * 60 * 24;

fn mutability_1() {
    let mut x = 5;
    println!("x starts of as {}", x);
    x = 6;
    println!("x is now {}", x);
}

fn main() {
    println!("Hello, world!");
    mutability_1();

    println!("One day in seconds {}", ONE_DAY_IN_SECONDS);
}