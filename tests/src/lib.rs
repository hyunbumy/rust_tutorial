#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// Each test is run in its own thread.
// Unit tests exist in the same file as the library and is just a child module with special
// annotation.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]  // This is a test
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_ne!(result, 5);
    }

    #[test]
    #[should_panic(expected = "match against the panic msg")]
    fn should_panic() {
        panic!("We can use the expected field to match against the panic msg");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller), "Larger count not hold smaller `{smaller:?}`");
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    // Using Result return for tests will allow us to use the `?` macro to return early if some
    // kind of variant is violated.
    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2,2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
