use std::io;

fn main() {
    println!("Hello, world!");
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("Inner scope: {}", x);
    }
    println!("Value of x: {}", x);
    let spaces = "    ";
    let spaces = spaces.len();
    // Error starts here
    // The same variable cannot hold different data types
    // let mut spaces = "    ";
    // spaces = spaces.len();
    // Use parse to parse String to integer
    let guess: i32 = "42".parse().expect("Not a number!");
    // char is Unicode Scalar Value which is 4 bytes in size
    let heart_eyed_cat = '😻';
    // Single element Tuple is possible
    let tup: (i32, ) = (500, );
    println!("{}", tup.0);
    let tup:(i32, &str, char, Option<f64>) = (1, "3", '2', None);
    let (x, y, z, n) = tup;
    let weekdays: [&str; 5] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    let bad_day = weekdays[0];
    println!("{}", bad_day);
    // Out of bound error may occur
    // let mut idx: String = String::new();
    // io::stdin().read_line(&mut idx).expect("Failed to read line");
    // let idx: usize = idx.trim().parse().expect("Not a number");
    // println!("{}", weekdays[idx]);
    // Function calls
    print_pair(5, 6);
    println!("{}", plus_one(7));
    let a = {
        let b = 3;
        b + 1
    };
    let cond = true;
    let num = if cond { 5 } else { 6 };
    let mut counter = 0;
    let result = loop {
        if counter > 9 {
            break counter * 2
        }
        counter += 1;
    };
    println!("{}", result);
    let mut count_up = 0;
    'label: loop {
        println!("outer: {count_up}");
        let mut count_down = 10;
        loop {
            println!("inner: {count_down}");
            if count_down <=9 {
                break
            }
            if count_up >= 2 {
                break 'label
            }
            count_down -= 1;
        }
        count_up += 1;
    }
    println!("end: {count_up}");
    let mut counter = 3;
    while counter >= 0 {
        println!("{counter}");
        counter -= 1;
    }
    let arr = [1, 2, 3, 4];
    for e in arr {
        println!("{e}");
    }
    for n in 0..4 {
        println!("n: {n}");
    }
    for n in (0..4).rev() {
        println!("n: {n}");
    }
}

fn print_pair(x: i32, y: i32) {
    println!("{} {}", x, y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
