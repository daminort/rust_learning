fn main() {
    let tuple = (12, 42.5, String::from("hello"));
    println!("Tuple: {:?}", tuple);

    let students = (String::from("Dan"), 5);
    let (name, grade) = students;
    println!("Name: {}, Grade: {}", name, grade);

    let users = (String::from("John"), 23);
    let user = users.0;
    let age = users.1;
    println!("User: {}, Age: {}", user, age);
}
