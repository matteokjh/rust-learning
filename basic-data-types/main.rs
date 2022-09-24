fn main() {
    // integer
    let x: i32 = 123456789; // integer -2^(n-1) ~ 2^(n-1)-1
    let _y: u32 = 123456789; // unsigned integer 0 ~ 2^n - 1
    println!("{}", x);

    // float
    let z: f32 = 10.9;
    println!("{}", z);

    // bool
    let b: bool = true;
    println!("{}", b);

    // char
    let str: char = 't';
    println!("{}", str);

    // tupple
    let mut tup: (i32, bool, char) = (999, false, 'k');
    tup.2 = 'j';
    println!("{}", tup.2);

    // array
    let mut arr: [i32; 5] = [-1, 2, 3, 4, 5];
    println!("{}", arr[0]);
    arr[0] = -2;
    println!("{}", arr[0]);

    let x: u8 = 4;
    let y: i32 = 4;
    println!("{}, {}", x, y);

    let x: i32 = "42".parse().expect("not a number");
    println!("{}", x);

    // range of integers
    let a = i8::MIN;
    println!("i8 min is {}", a);

    let a = i8::MAX;
    println!("i8 max is {}", a);

    let a = u8::MIN;
    println!("u8 min is {}", a);

    let a = u8::MAX;
    println!("u8 max is {}", a);

    let a = i16::MIN;
    println!("i16 min is {}", a);

    let a = i16::MAX;
    println!("i16 max is {}", a);

    let a = u16::MIN;
    println!("u16 min is {}", a);

    let a = u16::MAX;
    println!("u16 max is {}", a);

    let a = i32::MIN;
    println!("i32 min is {}", a);

    let a = i32::MAX;
    println!("i32 max is {}", a);

    let a = u32::MIN;
    println!("u32 min is {}", a);

    let a = u32::MAX;
    println!("u32 max is {}", a);

    let a = i64::MIN;
    println!("i64 min is {}", a);

    let a = i64::MAX;
    println!("i64 max is {}", a);

    let a = u64::MIN;
    println!("u64 min is {}", a);

    let a = u64::MAX;
    println!("u64 max is {}", a);

    let a = i128::MIN;
    println!("i128 min is {}", a);

    let a = i128::MAX;
    println!("i128 max is {}", a);

    let a = u128::MIN;
    println!("u128 min is {}", a);

    let a = u128::MAX;
    println!("u128 max is {}", a);

    let a = 57i8;
    println!("57u8 is {}", a);

    let a = 98222;
    println!("98_222 is {}", a);

    let a = 0xff;
    println!("0xff is {}", a);

    let a = 0o77;
    println!("0o77 is {}", a);

    let a = 0b1011;
    println!("0b1011 is {}", a);

    let a = b'A';
    println!("b'A' is {}", a);

    let tup: (i32, bool, char) = (8, false, 't');
    println!("tup.2 is {}", tup.2);
}
