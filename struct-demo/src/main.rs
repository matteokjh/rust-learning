struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // --snip--

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{}", user2.active);

    main2();
    main3();
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main2() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Color(r, g, b) = black;
    let r1 = black.0;
    let g1 = black.1;
    let b1 = black.2;

    println!("{},{},{}", r, g, b);
    println!("{},{},{}", r1, g1, b1);
}

struct User1 {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main3() {
    let user1 = User1 {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
