use std::io;

fn main() {
    loop {
        println!("");
        println!("Enter the N-th Fibonacci number:");
        let num = input_number();
        let result = get_next(num);

        println!("The {}th Fibonacci number is: {}", num, result);
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

fn get_next(num: i32) -> i32 {
    let mut res = 1;
    for element in 1..num + 1 {
        res = res * element;
    }

    res
}