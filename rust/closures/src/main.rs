use std::thread;

fn do_something(my_fn: FnOnce() -> u8) -> u8 {
    my_fn()
}

fn main() {
    println!("Hello, world!");

    let list = vec![1, 2, 3];

    let bar = do_something(move || list.len());
    println!("Foo: {}", bar);

    thread::spawn(move || println!("The list: {list:?}"))
        .join()
        .unwrap();
}
