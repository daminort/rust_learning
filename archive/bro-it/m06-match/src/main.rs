fn main() {
    let num = 45;
    match num {
        10 => println!("10 is now!"),
        23 => println!("45 is now!"),
        25..=50 => println!("10 < N < 50"),
        _ => println!("others"),
    };

    let n = 7;
    let x = match n {
        2 => 1,
        3 => 10,
        4..=10 => 20,
        _ => 0,
    };
    println!("x = {}", x);

    let is_ready: bool = false;
    let message = match is_ready {
        true => "Let's go!",
        false => "Wait please...",
    };
    println!("{}", message);
}
