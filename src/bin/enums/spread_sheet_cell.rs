pub enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn print_value(cell: SpreadsheetCell) {
    match cell {
        SpreadsheetCell::Int(i) => println!("int: {}", i),
        SpreadsheetCell::Float(f) => println!("float: {}", f),
        SpreadsheetCell::Text(t) => println!("text: {}", t),
    }
}
