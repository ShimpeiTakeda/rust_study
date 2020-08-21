use std::collections::HashMap;

fn main() {
    let text = "hello";

    let mut map = HashMap::new();

    for word in text.chars() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
