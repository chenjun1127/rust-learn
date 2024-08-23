pub fn vector_test() {
    let mut vector: Vec<i32> = Vec::new();
    vector = vec![1, 3, 4, 1];
    let mut a: Vec<i32> = Vec::new();
    a = vec![12, 3, 4];
    vector.append(&mut a);
    println!("{:?}", vector.get(0));
}
