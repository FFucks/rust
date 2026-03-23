pub fn run() {
    let x = Some(5);

    match x {
        Some(v) => println!("value: {v}"),
        None => println!("nothing"),
    }

    if let Some(v) = x {
        println!("if let: {v}");
    }

    let coin = Coin::Penny;
    println!("coin value: {}", value(coin));
}

enum Coin {
    Penny,
    Nickel,
}

fn value(c: Coin) -> u32 {
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 5,
    }
}