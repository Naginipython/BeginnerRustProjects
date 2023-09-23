pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: i32,
    height: i32,
}

#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn guess_width(&self, value: i32) -> bool {
        if value >= 100 {
            panic!("Guess value must be less than or equal to 100");
        } else if value < 0 {
            panic!("Guess value must be greater than or equal to 1");
        }

        self.width == value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_ne!(
            result, 
            5,
            "Result should not equal 5. Result: {}",
            result
        );
        assert_ne!(result, 3);
    }

    static LARGER: Rectangle = Rectangle {
        width: 8,
        height: 8,
    };
    static SMALLER: Rectangle = Rectangle {
        width: 5, 
        height: 2,
    };

    #[test]
    fn larger_can_hold_smaller() {
        assert!(
            LARGER.can_hold(&SMALLER),
            "Smaller rectangle cannot fit inside larger"
        );
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        assert!(
            !SMALLER.can_hold(&LARGER),
            "Smaller rectangle can fit inside larger"
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn guess_width_panics_large() {
        LARGER.guess_width(200);
    }

    #[test]
    #[should_panic(expected = "greater than or equal to 1")]
    fn guess_width_panics_small() {
        LARGER.guess_width(-1);
    }

    //Test that don't panic
    #[test]
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two doesn't equal four"))
        }
    }

    #[test]
    fn test_and_outputs() {
        println!("Output via running: cargo test -- --show-output");
        assert!(1+1 == 2);
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        println!("test is ignored unless explicitly called: cargo test -- --ignored");
        assert!(1+1 == 2);
    }
}
