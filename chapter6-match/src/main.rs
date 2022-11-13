enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let penny = value_in_cents(Coin::Penny);
    println!("The value of 1 penny is: {}", penny);

    let nickel = value_in_cents(Coin::Nickel);
    println!("The value of 1 nickel is: {}", nickel);

    let dime = value_in_cents(Coin::Dime);
    println!("The value of 1 dime is: {}", dime);

    let quarter = value_in_cents(Coin::Quarter);
    println!("The value of 1 quarter is: {}", quarter);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("This is 'five': {:?}", five);
    println!("This is 'six': {:?}", six);
    println!("This is 'none': {:?}", none);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
