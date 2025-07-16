mod enums;
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
    
    let op1 = enums::operation::Operation::Add;
    let result1 = op1.apply(5, 3);
    println!("result = {}", result1);
    let op2 = enums::operation::Operation::Subtract;
    let result2 = op2.apply(5, 3);
    println!("result2 = {}", result2);
}
