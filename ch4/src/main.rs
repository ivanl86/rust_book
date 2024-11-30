fn main() {
    let mut s1 = String::from("Hello");
    s1.push_str(", World");
    println!("{s1}");
    // 's1' is moved to 's2'
    let s2 = s1;
    println!("{s2}");
    // println!("{s1}"); // 's1' is no longer valid
    let x: i32 = 5;
    let y: i32 = x;
    println!("{x}");
    println!("{y}");
    // Copy data without move
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");
    // Move and copy work the same way when passing variables into functions
    let s1 = String::from("Hello");
    move_str(s1);
    // 's1' is invalid
    // println!("{s1}");
    let n = 300;
    copy_int(n);
    // n is still valid
    println!("{n}");
    let s3 = gives_str();
    println!("{s3}");
    let mut s4 = takes_n_gives(s3);
    // 's3' is moved to function take_n_give
    // println!("{s3}");
    println!("{s4}");
    // Reference does not take ownership
    let length = get_len(&s4);
    println!("Length of {s4} is {length}");
    add_str(&mut s4);
    println!("{s4}");
    let mut s = String::from("Hello again");
    // Cannot borrow mutable more than once at a time
    // let s1 = &mut s;
    // let s2 = &mut s;
    // println!("{s1},  {s2}");
        let mut s = String::from("Hello again");
    let s1 = &mut s;
    {
        // Multiple mutable borrow is allowed in different scopes
        let s2 = &mut s;
    }
    // Mutable and immutable borrow cannot occur in the same scope
    // let mut s = String::from("Hello again");
    // let s1 = &s;
    // let s2 = &s;
    // let s3 = &mut s;
    // println!("{s1}, {s2}, {s3}");

    // It compiles if last usage of the immutable occurs before the usage of mutable
    let mut s = String::from("Hello again");
    let s1 = &s;
    let s2 = &s;
    println!("{s1}, {s2}");
    let s3 = &mut s;
    println!("{s3}");
    let first = find_first(s3);
    println!("{first}");
}

fn move_str(s: String) {
    println!("{s}");
}

fn copy_int(i: i32) {
    println!("{i}");
}

fn gives_str() -> String {
    String::from("new string")
}

fn takes_n_gives(s: String) -> String {
    s
}

fn get_len(s: &String) -> usize {
    s.len()
}

fn add_str(s: &mut String) {
    s.push_str("additional");
}

// fn dangle() -> &String {
//    &String::from("Dangle")
// }

// fn no_dangle() -> String {
//    String::from("No dangle")
// }

// fn find_first(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &byte) in bytes.iter().enumerate() {
//         if byte == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

fn find_first(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
