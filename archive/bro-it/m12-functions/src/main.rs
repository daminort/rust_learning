fn main() {
    let name = String::from("Jane");
    hello(name);

    let res = sum(3, 4);
    println!("Res: {}", res);

    let (sum, sub, mult) = math(12, 85);
    println!("Sum: {}, Sub: {}, Mult: {}", sum, sub, mult);
}

fn hello(name: String) {
    println!("Hello, {}!", name);
}

fn sum(a: i8, b: i8) -> i8 {
    a + b
}

fn math(a: i32, b: i32) -> (i32, i32, i32) {
    (a + b, a - b, a * b)
}