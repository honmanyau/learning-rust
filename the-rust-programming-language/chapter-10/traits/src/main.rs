fn main() {
    let number_list = vec![42, 4, 2, 4242, 24];

    println!("The largest number is: {}", largest(&number_list));

    let char_list = vec!['n', 'y', 'p', 'a', 's', 'u'];

    println!("The largest char is : {}", largest(&char_list));

    println!("The largest number is: {}", largest_clone(&number_list));

    println!("The largest number is: {}", largest_ref(&number_list));
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    let mut index = 0;

    while index < list.len() {
        let item = list[index].clone();

        if item > largest {
            largest = item;
        }
        
        index += 1;
    }

    return largest
}

fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}