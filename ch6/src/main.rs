fn main() {
    let v4 = IpAddrType::V4(127, 0, 0, 0);
    let v6 = IpAddrType::V6(String::from("::1"));
    println!("{v4:?}");
    println!("{v6:?}");
    let msg = Message::Write(String::from("Hello"));
    msg.call();
    // let some_number = Some(5);
    // let some_char = Some('c');
    // let absent_number: Option<i32> = None;
    let penny = Coin::Penny;
    let penny_val = value_in_cents(penny);
    println!("The value of a penny is {penny_val} cent");
    let quarter = Coin::Quarter(UsState::Alaska);
    let quarter_val = value_in_cents(quarter);
    println!("The value of a quarter is {quarter_val} cents");
    let five = Some(5);
    let size = plus_one(five);
    let non = plus_one(None);
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    };
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}

#[derive(Debug)]
enum IpAddrType {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None
    }
}

// Option<T> enum
// enum Option<T> {
//     None,
//     Some(T),
// }
