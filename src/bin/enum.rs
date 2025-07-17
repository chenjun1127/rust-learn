mod enums;
use enums::color::*;
fn main() {
    // 定义一个Ok类型的Result，值为42
    let ok_result = Ok(42);
    // 调用common模块中的print_result函数，传入ok_result
    enums::common::print_result(ok_result);

    // 定义一个Err类型的Result，值为"Oops"
    let err_result = Err("Oops");
    // 调用common模块中的print_result函数，传入err_result
    enums::common::print_result(err_result);

    // 定义一个Some类型的Option，值为"ChatGPT"
    let name = Some("ChatGPT");
    // 调用common模块中的print_option函数，传入name
    enums::common::print_option(name);

    // 定义一个None类型的Option，值为空
    let none_name: Option<&str> = None;
    // 调用common模块中的print_option函数，传入none_name
    enums::common::print_option(none_name);
    println!("---------------------------");
    let op1 = enums::operation::Operation::Add;
    let result1 = op1.apply(5, 3);
    println!("result1 = {}", result1);
    let op2 = enums::operation::Operation::Subtract;
    let result2 = op2.apply(5, 3);
    println!("result2 = {}", result2);
    println!("---------------------------");
    let row = vec![
        enums::spread_sheet_cell::SpreadsheetCell::Int(3),
        enums::spread_sheet_cell::SpreadsheetCell::Float(10.12),
        enums::spread_sheet_cell::SpreadsheetCell::Text(String::from("blue")),
    ];

    for cell in row {
        enums::spread_sheet_cell::print_value(cell);
    }
    println!("---------------------------");
    let x = Number::One;
    let y = Color::Blue;

    // match
    match x {
        Number::Zero => println!("Zero"),
        Number::One => println!("One"),
        Number::Two => println!("Two"),
    }

    match y {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::Custom(10, 20, 30) => println!("Custom"),
        Color::Custom(r, g, b) => println!("Custom color RGB({}, {}, {})", r, g, b),
    }
    let red = Color::Red;
    let custom = Color::Custom(10, 20, 100);

    println!("Red: {}", red.to_hex()); // Red: #FF0000
    println!("Custom: {}", custom.to_hex()); // Custom: #0A1464
}
