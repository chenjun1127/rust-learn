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
}
