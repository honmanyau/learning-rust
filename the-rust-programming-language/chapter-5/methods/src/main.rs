fn main() {
    let rect = Rectangle {
        height: 24,
        width: 42
    };
    
    let another_rect = Rectangle {
        height: 12,
        width: 21,
    };

    let square = Rectangle::square(8);

    println!("{:#?}", rect);
    println!("The area of rect is: {}", rect.area());
    println!("The area of another_rect is: {}", another_rect.area());
    println!("rect can hold another_rect: {}", rect.can_hold(&another_rect));
    println!("{:#?}", square);
    println!("The area of square is: {}", square.area());
    println!("rect can hold square: {}", rect.can_hold(&square));
    println!("square can hold rect: {}", square.can_hold(&rect));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, target: &Rectangle) -> bool {
        target.height < self.height && target.width < self.width
    }
    
    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }
}
