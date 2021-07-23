fn main() {
    let x = 5;

    println!("The first declaration is: {}.", x);

    let x = x + 1;

    println!("The first shadowing is: {}.", x);

    let x = 'a';

    println!("The second shadowing is: {}.", x);
}
