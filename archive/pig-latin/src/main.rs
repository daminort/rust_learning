fn main() {
    let words = ["first", "apple", "delay", "event"];
    for w in words {
        println!("{}", to_latin(w));
    }
}

fn is_vowel(symbol: &char) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    vowels.contains(symbol)
}

fn to_latin(word: &str) -> String {
    let first = match word.chars().nth(0) {
        Some(value) => value,
        None => ' ',
    };

    match is_vowel(&first) {
        true => format!("{}-hay", word),
        false => {
            let res = match word.get(1..) {
                Some(s) => s,
                None => word,
            };
            format!("{}-{}ay", res, first)
        }
    }
}