use std::collections::HashMap;
use crate::department::Department;

#[derive(Debug)]
pub struct Organization {
  departments: HashMap<u32, Department>,
}

impl Organization {
  // private
  fn next_id(&self) -> u32 {
    let mut ids: Vec<&u32> = self.departments.keys().collect();
    let len = ids.len();
    if len == 0 {
      return 1
    }

    ids.sort();
    match ids.get(len - 1) {
      Some(id) => *id + 1,
      None => 1
    }
  }

  fn add_department(&mut self, dep: Department) {
    let id = self.next_id();
    self.departments.insert(id, dep);
  }

  fn find_department(&self, dep_name: &str) -> Option<&Department> {
    for (_, department) in &self.departments {
      if department.name == dep_name {
        return Some(department)
      }
    }

    None
  }

  // public
  pub fn new() -> Organization {
    Organization {
      departments: HashMap::new(),
    }
  }

  pub fn init(&mut self) {
    let mut direction = Department::new(String::from("Direction"));
    direction.hire(String::from("Jon Snow"));
    direction.hire(String::from("Daenerys Targaryen"));

    let mut sales = Department::new(String::from("Sales"));
    sales.hire(String::from("Arya Stark"));
    sales.hire(String::from("Sansa Stark"));
    sales.hire(String::from("Cersei Lannister"));

    self.add_department(direction);
    self.add_department(sales);
  }

  pub fn hire_to(&mut self, dep_name: &str, worker: String) {
    let mut dep = match self.find_department(dep_name) {
      Some(dep) => dep,
      None => {
        println!("Unable to find department \"{}\"", &dep_name);
        println!("Available departments are:");
        self.print_departments();

        return
      }
    };

    println!("Dep: {:?}, worker: {worker}", dep);
  }

  pub fn print_departments(&self) {
    for (id, department) in &self.departments {
      println!("{id} - {}", department.name);
    }
  }

  pub fn print_structure(&self) {
    for (_, department) in &self.departments {
      department.print();
    }
  }
}
