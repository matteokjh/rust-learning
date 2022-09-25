fn main() {
    println!("Hello, world!");

    // basic conditions
    let cond = 2 as f32 <= 2.2;
    println!("{}", cond);

    // compound conditions
    let cond2 = true && cond;
    let cond3 = false || cond;
    println!("{}", cond2);
    println!("{}", cond3);

    // if else else if
    // let food = "cookie";
    let food = "cake";
    if food == "cookie" {
        println!("food is {}", &food);
    } else {
        println!("food is not cookie, is {}", &food);
    }
}