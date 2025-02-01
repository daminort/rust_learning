fn main() {
    let limit: i8 = 18;
    let age: i8 = 17;
    if age >= limit {
        println!("You are an age of {} year! Welcome!", age);
    } else {
        println!("You are an age of {} year. Allowed {} only!", age, limit);
    }

    let flag = true;
    let num = if flag { 1 } else { 0 };
    println!("The value of num is: {}", num);
}
