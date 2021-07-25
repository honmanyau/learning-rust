fn main() {
    let up = Direction::Up;
    let down = Direction::Down;
    let left = Direction::Left;
    let right = Direction::Right;

    println!("{}", get_direction_symbol(up));
    println!("{}", get_direction_symbol(down));
    println!("{}", get_direction_symbol(left));
    println!("{}", get_direction_symbol(right));
}

fn get_direction_symbol(direction: Direction) -> String {
    match direction {
        Direction::Up => String::from("↑"),
        Direction::Down => String::from("↓"),
        Direction::Left => String::from("←"),
        Direction::Right => String::from("→"),
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}