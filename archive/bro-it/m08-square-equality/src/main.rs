use std::io;

fn main() {

    // ax^2 + bx + c = 0
    // D = b^2 - 4ac
    // > 0: (-b +/- D.sqrt()) / 2a
    // = 0: -b / 2a
    // < 0: none

    println!("Square equality exercise");

    let mut a_str = String::new();
    let mut b_str = String::new();
    let mut c_str = String::new();

    println!("Enter A:");
    match io::stdin().read_line(&mut a_str) {
        Ok(_) => {},
        Err(e) => println!("Input error: {}", e),
    };

    println!("Enter B:");
    match io::stdin().read_line(&mut b_str) {
        Ok(_) => {},
        Err(e) => println!("Input error: {}", e),
    };

    println!("Enter C:");
    match io::stdin().read_line(&mut c_str) {
        Ok(_) => {},
        Err(e) => println!("Input error: {}", e),
    };

    let a: f64 = match a_str.trim().parse() {
        Ok(num) => num,
        Err(e) => panic!("{}", e),
    };

    let b: f64 = match b_str.trim().parse() {
        Ok(num) => num,
        Err(e) => panic!("{}", e),
    };

    let c: f64 = match c_str.trim().parse() {
        Ok(num) => num,
        Err(e) => panic!("{}", e),
    };

    let d = (b * b) - 4.0 * a * c;

    match d {
        0.0 => {
            let x = (-b) / (2.0 * a);
            println!("D = 0, solution: {}", x);
        },
        0.0.. => {
            let x1 = ((-b) + d.sqrt()) / (2.0 * a);
            let x2 = ((-b) - d.sqrt()) / (2.0 * a);
            println!("D > 0, solutions: {} and {}", x1, x2);
        }
        _ => println!("D < 0, there is no solution"),
    }
}
