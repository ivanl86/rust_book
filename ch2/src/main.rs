use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Hello, world!");
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);
    // println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        let guess: u16 = match guess.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}