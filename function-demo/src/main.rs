fn main() {
    println!("Hello, world!");
    let tup: (i32, bool, char) = (-1, true, 'A');
    test(tup);
    println!("{}", tup.2);
    println!("{}", test2(5));
}

fn test(mut i: (i32, bool, char)) {
    i.2 = 'B';
    println!("{}", i.2);
}

fn test2(x: i32) -> i32 {
    x + 1
}