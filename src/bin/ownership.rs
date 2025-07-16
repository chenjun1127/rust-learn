fn main() {
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // let s3 = s2;
    // println!("{}, world!", s3);
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}
fn calculate_length(s: &str) -> usize {
    s.len()
}
