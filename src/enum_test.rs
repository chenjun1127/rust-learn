enum Coin {
    Penny,
    Nickel,
    Dimme,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dimme => 10,
    }
}
