use std::error::Error;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    // let mut v = vec![1, 2, 3, 4, 5];
    // v[99];
    // BACKTRACE=1 cargo run for backtrace message

    // let f_result = File::open("hello.txt");
    
    // let f = match f_result {
    //     Ok(f) => f,
    //     Err(e) => match e.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(f) => f,
    //             Err(e) => panic!("Problem creating file: {}", e),
    //         }
    //         _ => panic!("Problem opening file: {}", e),
    //     }
    // };
    
    // let f = read_file();

    let f_result = File::open("hello.txt")?;

    Ok(())
}

fn read_file() -> Result<String, io::Error> {
    let mut file = String::new();
    File::open("hello.txt")?.read_to_string(&mut file)?;
    Ok(file)
}
