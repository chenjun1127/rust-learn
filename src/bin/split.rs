// mod split;
fn main(){
    // 示例：分割字符串
    let text = "Hello, world! This is a test.";
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("Words: {:?}", words);

    // 使用 split 方法分割字符串
    let csv = "apple,banana,cherry";
    let fruits: Vec<&str> = csv.split(',').collect();
    println!("Fruits: {:?}", fruits);

    // 使用 splitn 方法限制分割次数
    let limited_split: Vec<&str> = text.splitn(3, ' ').collect();
    println!("Limited split: {:?}", limited_split);
}