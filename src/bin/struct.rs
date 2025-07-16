#[derive(Debug)]
struct User {
    name: String,
    active: bool,
    email: String,
    sing_in_count: u64,
    age: i32,
}
#[derive(Debug)]
struct Rect(u32, u32);
impl Rect {
    fn area(&self) -> u32 {
        self.0 * self.1
    }
    fn width(&self) -> bool {
        self.0 > self.1
    }
    fn can_hold(&self, other: &Rect) -> bool {
        self.0 > other.0 && self.1 > other.1
    }
    fn square(size: u32) -> Rect {
        Rect(size, size)
    }
}
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        active: true,
        sing_in_count: 1,
        age: 32,
        name: String::from("some name"),
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(String::from("example@example.com"), 32);

    println!("{:#?}", user2);

    println!("{:?}", user2);
    println!("User name: {}", user2.name);
    println!("Active: {}", user2.active);
    println!("Sign in count: {}", user2.sing_in_count);
    println!("Age: {}", user2.age);
    println!("Email: {}", user2.email);

    let rect = Rect(30, 50);
    println!("Area: {}", area(&rect));
    println!("rect is {:?}", rect);
    println!("width is {}", rect.width());
    let rect1 = Rect(30, 50);
    let rect2 = Rect(10, 30);
    println!("rect1 is {:?}", rect1.area());
    println!("rect2 is {:?}", rect2.area());
    println!("Can rect1 hold rect2 {}", rect1.can_hold(&rect2));
    let rect3 = Rect(40, 40);
    println!("Can rect1 hold rect3 {}", Rect::square(40).can_hold(&rect3));
}
fn build_user(email: String, age: i32) -> User {
    User {
        email,
        active: true,
        sing_in_count: 1,
        age,
        name: String::from("some name"),
    }
}
fn area(rect: &Rect) -> u32 {
    rect.0 * rect.1
}
