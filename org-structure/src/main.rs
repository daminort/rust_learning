mod organization;
mod department;
mod flow;
mod input;

use organization::Organization;

fn main() {
  let mut org = Organization::new();
  org.init();

  loop {
    flow::select_mode(&mut org);
  }
}
