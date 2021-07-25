fn main() {
    let up1 = DirectionKeyOld {
        direction: Direction::Up,
        key: String::from("↑"),
    };
    let up2 = DirectionKey::Up(String::from("↑"));


    println!("{:#?}", up1);
    println!("{:#?}", up2);
    up2.pretty_print();
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum DirectionKey {
    Up(String),
    Down(String),
    Left(String),
    Right(String),
}

#[derive(Debug)]
struct DirectionKeyOld {
    direction: Direction,
    key: String,
}

impl DirectionKey {
    fn pretty_print(&self) {
        println!("{:#?}", self);
    }
}