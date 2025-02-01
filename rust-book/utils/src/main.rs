use std::io;

fn main() {
    println!("Hello, world!");
}

fn input_number() -> i32 {
    loop {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line");

        match buf.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        }
    }
}