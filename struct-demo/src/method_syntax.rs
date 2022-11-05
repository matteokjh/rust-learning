struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn get_area_demo() {
    let rect1 = Rectangle {
        width: 50,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 49,
        height: 51,
    };

    let rect3 = Rectangle::square(44);

    println!("area: {}", rect1.area());

    println!("can_hold: {}", rect1.can_hold(&rect2));

    println!("rect3 {}", rect3.area())
}
