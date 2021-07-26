fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(42); // Adding element.

    println!("The value at index {} of v is: {}", 0, v[0]);

    let mut v = vec![1, 2, 3]; // Type infrencin
    
    v.push(4); // Adding element.

    println!("The value at index {} of v is: {}", 3, v[3]);

    let index_two = &v[2];

    println!("The value at index 2 of v is: {}", index_two);

    let index_two = v.get(2);

    match index_two {
        Some(x) => println!("The value at index 2 of v is: {}", x),
        None => println!("No value found at the given index!")
    };
}
