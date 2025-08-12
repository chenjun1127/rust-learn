// Rust 没有传统的 interface，
// 但 trait = 接口的超集，可以完全替代接口，还能做更多接口做不到的事。
trait Fly {
    fn fly(&self);
}
struct Bird;
impl Fly for Bird {
    fn fly(&self) {
        println!("The bird is flying!");
    }
}
// 物品：飞机
struct Airplane;

// 贴标签：飞机也会飞
impl Fly for Airplane {
    fn fly(&self) {
        println!("飞机在天上飞~");
    }
}

fn main() {
    let b = Bird;
    let p = Airplane;
    b.fly(); // 鸟在天上飞~
    p.fly(); // 飞机在天上飞~
}
