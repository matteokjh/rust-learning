use std::io;

fn main() {
    println!("Hello, world!");
    let x = 9.4f32;
    let y = 10.9;

    let z = x + y;
    println!("{}", z);

    let a = 9_i8;
    let b = 10 as f32;
    let c = (a as f32) / b;
    println!("{}", c);

    let d = (i32::MAX as i64) + 1;
    let e = 10_i32;
    let f = d as i32 / e; // overflow
    println!("{}", f);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("expected to read line.");
    let int_input: i64 = input.trim().parse().unwrap();
    println!("{}", int_input);
}
