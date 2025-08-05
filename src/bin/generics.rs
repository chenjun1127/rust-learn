#[derive(Debug)]
struct Point<T, R> {
    x: T,
    y: R,
}
fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point {
        x: "hello",
        y: 10.4,
    };
    println!("{:?}", p1);
    println!("{:?},{:?}", p2.x,p2.y);
    let arr = [1, 2, 3, 4, 5];
    let max_value = max(&arr);
    println!("The maximum value in the array is: {}", max_value);
    let p3 = Point::new(5, 10.4);
    println!("Point p3: x = {}, y = {}", p3.x(), p3.y);
    
}
fn max(arr: &[i32]) -> i32 {
    let mut max = arr[0];
    for &item in arr.iter() {
        if item > max {
            max = item;
        }
    }
    max
}

impl<T> Point<T, f32> {
    fn new(x: T, y: f32) -> Self {
        Point { x, y }
    }
    fn x(&self) -> &T {
        &self.x
    }
    
}