mod organization;
mod department;
mod flow;

use organization::Organization;

fn main() {
  let mut org = Organization::new();
  org.init();

  loop {
    flow::select_mode(&mut org);
  }
}
