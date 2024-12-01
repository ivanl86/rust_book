use std::collections::HashMap;

fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(6);
    v.push(7);
    v.push(8);
    let third = &v[2];
    println!("The third element is {third}");
    let third = v.get(2);
    match third {
        Some(v) => println!("The third element is {v}"),
        None => println!("There is no third element!"),
    }
    let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    v.push(6);
    // Mutable and immutable cannot be in the same scope still apply
    // println!("The first element is {first}"); // Error here
    let first = &v[0];
    println!("The first element is {first}");
    for i in &v {
        println!("{i}");
    }
    for i in &mut v {
        *i += 10;
    }
    for i in &v {
        println!("{i}");
    }
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(5.12),
        SpreadSheetCell::Text(String::from("rust")),
    ];
    for cell in &row {
        println!("{cell:?}")
    }

    // String literal
    let data = "initial contents";
    // Owned string
    // New empty string
    let s = String::new();
    // With initial data
    let s = String::from("initial contents");
    let s = data.to_string(); // Same as String::from()
    println!("{s}");
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}");
    let s1 = String::from("Hello, ");
    let s2 = String::from(" world");
    let s3 = s1 + &s2;
    // + operator takes self + &str
    // println!("{s1}"); // s1 is moved to s3
    println!("{s3}");
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
    let a = "a";
    for i in 0..26 {
        print!("{}", (a.bytes().nth(0).unwrap() + i) as char);
    }
    println!();
    let c = 'A';
    for i in 0..26 {
        print!("{}", ((c as u8) + i) as char);
    }
    println!();
    let d = b'a' - b'A';
    for i in 0..26 {
        let b = c as u8;
        let l = (b + d + i) as char;
        print!("{l}");
    }
    println!();
    let hello = "Здравствуйте";
    // let s = &hello[0..3];
    // println!("{s}");
    for c in hello.chars() {
        print!("{c} ");
    }
    println!();
        for c in hello.bytes() {
        print!("{c} ");
    }
    println!();

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10); // Insert new key and value
    scores.insert(String::from("Blue"), 25); // Overriding the existing value
    scores.insert(String::from("Red"), 50);
    scores.entry(String::from("Red")).or_insert(40); // Insert new key and value else do nothing
    scores.entry(String::from("Yello")).or_insert(80); // Insert new key and value else do nothing
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).unwrap_or(&0); // If the key exits get the value else 0
    println!("Team {team_name} : {score} pts");
    for (k, v) in scores {
        println!("{k} : {v}");
    }
    let text = "Hello world it is a wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    for (k, v) in map {
        println!("{k} : {v}");
    }
}

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
