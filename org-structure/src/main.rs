mod organization;
mod department;

use organization::Organization;

fn main() {
    let mut org = Organization::new();

    org.init();
    org.print_departments();
    org.print_structure();

    org.hire_to("Sales", String::from("Jade"));
}
