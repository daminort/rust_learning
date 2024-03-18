use std::collections::HashMap;
use crate::department::Department;

#[derive(Debug)]
pub struct Organization {
  departments: HashMap<usize, Department>,
}

impl Organization {
  // private
  fn next_id(&self) -> usize {
    let mut ids: Vec<&usize> = self.departments.keys().collect();
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

  fn add_department(&mut self, dep: Department) -> usize {
    let id = self.next_id();
    self.departments.insert(id, dep);

    id
  }

  fn find_department(&mut self, dep_name: &str) -> Option<&mut Department> {
    let mut id: usize = 0;
    let cloned_deps = &self.departments.clone();
    for (current_id, department) in cloned_deps {
      if department.name == dep_name {
        id = *current_id;
        break;
      }
    }

    if id == 0 {
      return None
    }
    
    self.departments.get_mut(&id)
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

  pub fn get_department(&self, id: usize) -> Option<&Department> {
    self.departments.get(&id)
  }

  pub fn hire_to(&mut self, dep_name: &str, worker: String) {
    match self.find_department(dep_name) {
      Some(dep) => {
        dep.hire(worker);
      }
      None => {
        let mut new_dep = Department::new(String::from(dep_name));
        new_dep.hire(worker);
        self.add_department(new_dep);
      }
    }
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
