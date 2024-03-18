use std::io;
use std::process::exit;
use crate::organization::Organization;

pub fn select_mode(org: &mut Organization) {
  println!("--------------------------------");
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
    "1" => print_org(&org),
    "2" => print_dep(&org),
    "3" => hire(org),
    _ => {
      println!();
      select_mode(org);
    }
  }
}

fn print_org(org: &Organization) {
  org.print_structure();
}

fn print_dep(org: &Organization) {
  println!("-----");
  println!("Please, select the department:");
  org.print_departments();

  let dep_id = loop {
    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read the line");

    match input.trim().parse() {
      Ok(value) => break value,
      Err(_) => {
        println!("Enter valid number");
        continue;
      },
    }
  };

  match org.get_department(dep_id) {
    Some(dep) => dep.print(),
    None => {
      println!();
      println!("Unable to find department with ID {}", dep_id);
      print_dep(org);
    }
  }
}

fn hire(org: &mut Organization) {
  println!("-----");
  println!("Template to hire: \"Add NAME to DEPARTMENT\"");

  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read the line");

  let input = input.trim();
  let parts: Vec<&str> = input.split(' ').collect();

  let name = match parts.get(1) {
    Some(value) => value,
    None => "Noname"
  };

  let dep_name = match parts.get(3) {
    Some(value) => value,
    None => "Bench"
  };

  org.hire_to(dep_name, String::from(name));
}