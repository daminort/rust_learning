use std::io;

pub fn read_string() -> String {
  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read the line");

  let input = input.trim();

  String::from(input)
}

pub fn read_number() -> usize {
  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read the line");

  let input = input.trim();

  input.trim().parse().unwrap_or_else(|_| 0)
}