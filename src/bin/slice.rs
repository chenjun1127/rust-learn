fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
    let bytes = s.as_bytes();
    println!("{:?}", bytes);
    println!("{}", first_word(&s));
    let s = String::from_utf8(bytes.to_vec()).unwrap();
    println!("{}", s);
}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
