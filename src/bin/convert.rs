use std::convert::TryFrom;

fn main() {
    // Number 可以直接 .into()
    let n: Number = 10.into();
    println!("Number: {}", n.value);

    // EvenNumber 需要 try_into()，可能失败
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    match result {
        Ok(even) => println!("Even number: {}", even.value),
        Err(_) => println!("Not an even number"),
    }

    // 尝试失败情况
    let result2: Result<EvenNumber, ()> = 7i32.try_into();
    println!("Result2 is_ok: {}", result2.is_ok());
}

struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value }
    }
}

struct EvenNumber {
    value: i32,
}

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber { value })
        } else {
            Err(())
        }
    }
}
