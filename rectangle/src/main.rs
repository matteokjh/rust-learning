fn main() {
    let rect_width = 20;
    let rect_height = 30;
    let area = calculate_rectangle_area(rect_width, rect_height);
    println!("rect area is {}", area);

    let rect = (20, 30);
    let area = calculate_rectangle_area_v2(rect);
    println!("rect area is {}", area);

    let rect = Rectangle {
        width: dbg!(20 * 2),
        height: 30,
    };
    let area = calculate_rectangle_area_v3(&rect);
    println!("rect is {:#?}", rect);
    println!("rect area is {}", area);
    dbg!(&rect);
}

fn calculate_rectangle_area(width: u32, height: u32) -> u32 {
    width * height
}

fn calculate_rectangle_area_v2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn calculate_rectangle_area_v3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
