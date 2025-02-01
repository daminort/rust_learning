fn main() {
    math();
    tuples();
    arrays();
    functions();
    loops();
    loop_for_1();
    loop_for_2();
}

fn print_title(title: &str) {
    println!("");
    print!("----- {} ", title);

    let dashes = "-".repeat(75 - title.len());
    println!("{}", dashes);
}

fn math() {
    print_title("Math");

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("sum: 5 + 10 = {}", sum);
    println!("difference: 95.5 - 4.3 = {}", difference);
    println!("product: 4 * 30 = {}", product);
    println!("quotient: 56.7 / 32.2 = {}", quotient);
    println!("truncated: -5 / 3 = {}", truncated);
    println!("remainder: 43 % 5 = {}", remainder);
}

fn tuples() {
    print_title("Tuples");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;

    println!("The value of Y is: {}", y);
    println!("The value of Z is: {}", tup.2);
}

fn arrays() {
    print_title("Arrays");

    let months = [
        "January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"
    ];

    println!("Months: {:?}", months);
}

fn functions() {
    print_title("Functions");

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    let five = add_one(4);
    println!("The value of five is: {}", five);
}

fn loops() {
    print_title("Loops");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn loop_for_1() {
    print_title("Loops: For #1");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn loop_for_2() {
    print_title("Loops: For #2");

    for num in (1..5).rev() {
        println!("num = {}", num);
    }
}