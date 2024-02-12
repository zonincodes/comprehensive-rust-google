#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum usState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(usState),
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i +1)
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(usState::Alabama) => 25,
        _ => 0,
    }
}
use std::{collections::HashMap, fmt::Debug};

fn vectors() {
    let mut v: Vec<i32> = Vec::new();
    let v1: Vec<i32> = vec![1, 3, 4];

    for i in 5..=8 {
        v.push(i);
    }
    for i in v1 {
        println!("{i}");
    }

    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];

    for i in row {
        println!("{:#?}", i);
    }
}


fn hash_map() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}


fn main() {
    let home: IpAddr= IpAddr::V4(String::from("127.0.0.1"));
    let loopback: IpAddr = IpAddr::V6(String::from("::1"));

    println!("{:?} {:?}", home, loopback);
    println!("Penny is worth: ${}", value_in_cents(Coin::Penny));

    let five: Option<i32> = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);

    println!("{:?}", five);

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to  be {}", max);
    }

    let mut count = 0;
    let coin = Coin::Quater(usState::Alabama);
    if let Coin::Quater(state) = coin {
        println!("State quater from {:?}!", state);
    } else {
        count  +=  1;
    }
}