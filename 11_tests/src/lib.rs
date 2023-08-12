pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_ne!(result, 5);
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
}
