use std::collections::HashMap;

fn main() {
    let mut nonsensical = HashMap::new();

    nonsensical.insert(String::from("nyan"), 24);
    nonsensical.insert(String::from("pasu"), 42);

    println!("{:?}", nonsensical);

    let words = vec!("nyan", "pasu");
    let nums = vec!(24, 42);
    let mut nonsensical: HashMap<_, _> =
        words.into_iter().zip(nums.into_iter()).collect();

    println!("{:?}", nonsensical);

    // Accessing values
    for (key, value) in &nonsensical {
        println!("{}: {}", key, value);
    }

    let value = nonsensical.get("nyan");

    match value {
        Some(x) => println!("{}", x),
        None => println!("No value found")
    }

    // Overwriting values
    nonsensical.insert("nyan", 42);
    nonsensical.insert("pasu", 24);

    println!("{:?}", nonsensical);

    // Inseerting if no matching key
    nonsensical.entry("nyanpasu").or_insert(4224);

    println!("{:?}", nonsensical);

    nonsensical.entry("nyanpasu").or_insert(2112);

    println!("{:?}", nonsensical);
}
