use std::io;

fn main() {
    let mut name = String::new();
    match io::stdin().read_line(&mut name) {
        Ok(_) => {
            println!("Hello, {}!", name);
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
