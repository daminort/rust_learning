use std::io;
use std::cmp::Ordering;
use rand::Rng;

const LIMIT: u32 = 100;

fn main() {
    println!("Guess the number!");
    println!("Enter your number (from 1 to {LIMIT}):");

    let secret = rand::thread_rng().gen_range(1..=LIMIT);
    let mut steps: u8 = 1;

    loop {        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please, enter a valid number");
                continue;
            },
        };

        match guess.cmp(&secret) {
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You won!");
                break;
            },
        }

        steps += 1;
    }

    println!("Steps: {steps}");
}
