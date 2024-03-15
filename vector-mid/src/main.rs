use std::collections::HashMap;

fn main() {
    let v5 = vec![8, 2, 300, 1, 15, 20, 11, 2, 40, 15, 60, 32, 15, 112, 87, 1, 12];
    let v4 = vec![50, 3, 68, 10, 3, 12, 60, 51, 3, 42];

    println!("Median");
    match median(&v5) {
        Some(value) => println!("median of v5: {}. List: {:?}", value.1, value.0),
        None => println!("v5 is empty"),
    }

    match median(&v4) {
        Some(value) => println!("median of v4: {}. List: {:?}", value.1, value.0),
        None => println!("v4 is empty"),
    }

    println!("");
    println!("Frequency");

    match frequency(&v5) {
        Some(value) => println!("frequency of v5: {:?}", value),
        None => println!("v5 is empty"),
    }

    match frequency(&v4) {
        Some(value) => println!("frequency of v4: {:?}", value),
        None => println!("v4 is empty"),
    }
}

fn median(v: &Vec<i32>) -> Option<(Vec<i32>, f32)> {
    let len = v.len();
    if len == 0 {
        return None;
    }

    let mut sorted = v.clone();
    sorted.sort();

    if len % 2 > 0 {
        let value = sorted[len / 2] as f32;
        return Some((sorted, value));

    } else {
        let min = len / 2;
        let max = min + 1;
        let value = (sorted[min] + sorted[max]) as f32 / 2.0;

        return Some((sorted, value));
    }
}

fn frequency(v: &Vec<i32>) -> Option<HashMap<i32, i32>> {
    if v.len() == 0 {
        return None;
    }

    let mut map = HashMap::new();
    for i in v {
        let count = map.entry(*i).or_insert(0);
        *count += 1;
    }

    Some(map)
}