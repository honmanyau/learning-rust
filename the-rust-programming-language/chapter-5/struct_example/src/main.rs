fn main() {
    example1();
    example2();
    example3();
    
    let rect = Rectangle {
        height: 24,
        width: 42
    };

    println!("{:#?}", rect);
}

fn example1() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width, height)
    );
}

fn example2() {
    let rect = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect)
    );
}

fn example3() {
    let rect = Rectangle {
        height: 50,
        width: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect) // Passing by reference so example3() retains ownership.
    );
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}