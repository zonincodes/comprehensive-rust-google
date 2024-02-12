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
    vectors();
    hash_map();

    let v: Vec<i32> = vec![1, 2, 3];

    v[99];
}
