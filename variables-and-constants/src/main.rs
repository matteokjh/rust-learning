fn main() {
    // by default, variable is immutable.
    let mut x = 4;
    println!("x is {}", x); // 4
    x = 5;
    println!("x is {}", x); // 5

    // overwrite
    let y = 4;
    println!("y is {}", y); // 4
    let y = y + 1;
    println!("y is {}", y); // 5

    // block scope
    let z = 4;
    {
        let z = z + 2; // 6
        println!("z is {}", z);
    }
    let z = z + 1; // 5
    println!("z is {}", z);

    // change type
    let a = 4;
    println!("a: {}", a);

    let a = "aaa";
    println!("a: {}", a);

    // constants
    const TEST: u32 = 60 * 60;
    println!("{}", TEST);
}
