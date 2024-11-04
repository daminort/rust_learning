fn main() {

    // loop
    let mut num = 0;
    loop {
        if num > 10 {
            break;
        }

        println!("Num is {}", num);
        num += 1;
    }

    // while
    println!("------------");
    while num <= 20 {
        if num % 2 == 0 {
            println!("Num is {}", num);
        }
        num += 1;
    }

    // for
    println!("------------");
    for i in 0..11 {
        if i % 2 == 0 {
            println!("Num is {}", i);
        }
    }
}
