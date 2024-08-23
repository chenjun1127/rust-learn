fn calc_area(width: u32, height: u32) -> u32 { width * height }

fn calc_area_2(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}

fn calc_area_3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn test_area() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        calc_area(30, 50)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        calc_area_2(rect2)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        calc_area_3(&rect1)
    );
    println!("rect1 is {:#?}", rect1);
}