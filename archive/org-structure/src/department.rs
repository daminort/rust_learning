#[derive(Debug, Clone)]
pub struct Department {
  pub name: String,
  workers: Vec<String>,
}

impl Department {
  pub fn new(name: String) -> Self {
    Department {
      name,
      workers: vec![],
    }
  }

  pub fn hire(&mut self, name: String) {
    self.workers.push(name);
    self.workers.sort();
  }

  pub fn print(&self) {
    println!();
    println!("Department: \"{}\"", &self.name);
    println!("Workers: {:?}", self.workers);
  }
}