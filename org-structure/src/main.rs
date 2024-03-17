mod organization;
mod department;

use organization::Organization;

fn main() {
    let mut org = Organization::new();

    org.init();
    org.print_departments();
    org.print_structure();

    println!();
    println!("-------------------");

    org.hire_to("Warehouse", String::from("Jade"));
    org.hire_to("Warehouse", String::from("Aaron"));
    org.print_structure();
}
