#[derive(Debug)]
pub struct Department {
  pub name: String,
  users: Vec<String>,
}

impl Department {
  pub fn new(name: String) -> Self {
    Department {
      name,
      users: vec![],
    }
  }

  pub fn hire(&mut self, name: String) {
    self.users.push(name);
  }

  pub fn print(&self) {
    let mut sorted = self.users.clone();
    sorted.sort();

    println!("");
    println!("Department: \"{}\"", &self.name);
    println!("Users: {:?}", sorted);
  }
}