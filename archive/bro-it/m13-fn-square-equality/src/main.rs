use std::io;

// ax^2 + bx + c = 0
// D = b^2 - 4ac
// > 0: (-b +/- D.sqrt()) / 2a
// = 0: -b / 2a
// < 0: none

fn main() {
    println!("Square equality exercise");

    let a = read_number(String::from("Enter A"));
    let b = read_number(String::from("Enter B"));
    let c = read_number(String::from("Enter C"));

    let d = calc_dmn(a, b, c);

    resolve(a, b, d);
}

fn read_number(msg: String) -> f64 {
    let mut num_str = String::new();

    println!("{}", msg);
    match io::stdin().read_line(&mut num_str) {
        Ok(_) => {},
        Err(e) => println!("Input error: {}", e),
    };

    let num: f64 = match num_str.trim().parse() {
        Ok(num) => num,
        Err(e) => panic!("{}", e),
    };

    num
}

fn calc_dmn(a: f64, b: f64, c: f64) -> f64 {
    let d = (b * b) - 4.0 * a * c;
    d
}

fn resolve(a: f64, b: f64, d: f64) {
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
