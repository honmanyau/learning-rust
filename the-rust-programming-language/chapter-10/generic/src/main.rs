fn main() {
    let number_list = vec![42, 4, 2, 4242, 24];

    println!("The largest number is: {}", largest_i32(&number_list));

    let char_list = vec!['n', 'y', 'p', 'a', 's', 'u'];

    println!("The largest char is : {}", largest_char(&char_list));
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest= list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

