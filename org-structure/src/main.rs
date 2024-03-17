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

  // org.print_departments();
  // org.print_structure();
  //
  // println!();
  // println!("-------------------");
  //
  // org.hire_to("Warehouse", String::from("Jade"));
  // org.hire_to("Warehouse", String::from("Aaron"));
  // org.print_structure();
}
