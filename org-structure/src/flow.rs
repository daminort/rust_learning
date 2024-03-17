use std::io;
use std::process::exit;
use crate::organization::Organization;

pub fn select_mode(org: &mut Organization) {
  println!();
  println!("Please, select the working mode:");
  println!("1 - print organization structure");
  println!("2 - print department structure");
  println!("3 - hire a new worker");
  println!("0 - exit program");
  println!();
  println!("Your choice: ");

  let mut mode = String::new();
  io::stdin()
    .read_line(&mut mode)
    .expect("Failed to read the line");

  let mode = mode.trim();
  match mode {
    "0" => exit(0),
    "1" => print_org(org),
    _ => {
      println!();
      select_mode(org);
    }
  }
}

fn print_org(org: &mut Organization) {
  org.print_structure();
}