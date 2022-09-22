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
    println!("{}, {}", x, y)

}