struct Rectangle {
    height: i32,
    width: i32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn it_does_not_work() {
    //     panic!("Wah!");
    // }

    #[test]
    fn larger_can_hold_smaller () {
        let larger = Rectangle {
            height: 100,
            width: 100,
        };
        let smaller = Rectangle {
            height: 99,
            width: 99,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger () {
        let larger = Rectangle {
            height: 100,
            width: 100,
        };
        let smaller = Rectangle {
            height: 99,
            width: 99,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn cannot_hold_equal () {
        let rect_a = Rectangle {
            height: 100,
            width: 100,
        };
        let rect_b = Rectangle {
            height: 100,
            width: 100,
        };

        assert!(!rect_a.can_hold(&rect_b));
    }

    #[test]
    fn fail_with_message() {
        assert!(
            2 + 4 == 42,
            "The numbers {} and {} do not add up to 42",
            2,
            4
        );
    }

    #[test]
    #[should_panic(expected = "Wah!")]
    fn panic_inducing_function() {
        panic!("Wah!");
    }
}
