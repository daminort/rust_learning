#[derive(Debug)]
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
  }

  pub fn print(&self) {
    let mut sorted = self.workers.clone();
    sorted.sort();

    println!("");
    println!("Department: \"{}\"", &self.name);
    println!("workers: {:?}", sorted);
  }
}