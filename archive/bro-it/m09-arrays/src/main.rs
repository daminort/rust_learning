fn main() {
    let mut arr = [1, 2, 3, 4, 5, 6, 7, 8];
    println!("{:?}", arr);
    println!("{}", arr[2]);

    arr[2] = 10;
    println!("{:?}", arr);

    let nums = [2; 10];
    println!("{:?}", nums);

    println!("---");
    for i in arr.iter() {
        println!("{}", i);
    }

    println!("---");
    for i in 0..arr.len() {
        println!("{}", arr[i]);
    }

    println!("---");
    let mut i = 0;
    while i < arr.len() {
        println!("{}", arr[i]);
        i += 1;
    }

    println!("---");
    for i in arr.iter() {
        if i % 2 == 0 {
            println!("{}", i);
        }
    }

    println!("--- Duplicates (works wrong!) ---");
    let origin = [7, 1, 2, 9, 3, 2, 4, 5, 6, 7, 7, 8, 9, 9];
    let mut i = 0;
    while i < origin.len() {
        let mut j = i + 1;
        while j < origin.len() {
            if origin[i] == origin[j] {
                println!("{}", origin[i]);
            }
            j += 1;
        }
        i += 1;
    }
}
