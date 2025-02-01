use std::io;

fn main() {
    println!("Temperature Converter");

    loop {
        match select_source() {
            1 => {
                println!("Please enter a temperature value in Celsius:");
                let num = input_number();
                celsius_to_fahrenheit(num);
            }
            2 => {
                println!("Please enter a temperature value in Fahrenheit:");
                let num = input_number();
                fahrenheit_to_celsius(num);
            }
            _ => {
                break;
            }
        }
    }
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

fn select_source() -> u8 {
    println!("Select a source:");
    println!("1 - Celsius");
    println!("2 - Fahrenheit");
    println!("0 - Exit");

    loop {
        match input_number() {
            1 => return 1,
            2 => return 2,
            0 => return 0,
            _ => {
                println!("Please make your choice!");
            }
        }
    }
}

fn celsius_to_fahrenheit(num: i32) {
    let res = (num * 9 / 5) + 32;
    println!("{} Celsius -> {} Fahrenheit", num, res);
}

fn fahrenheit_to_celsius(num: i32) {
    let res = (num - 32) * 5 / 9;
    println!("{} Fahrenheit -> {} Celsius", num, res);
}
