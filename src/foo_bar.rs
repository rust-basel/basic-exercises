/*
FooBar:
The Function/Method should return Foo, when a number is dividable by 3
The Function/Method should return Bar, when a number is dividable by 5
The Function/Method should return FooBar, when a number is dividable by 5 and 3
The Function/Method should return Nothing, when none of the above hold
*/
fn foobar(input: u32) -> Option<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::foo_bar::foobar;

    #[test]
    fn returns_foo_when_dividable_by_3() {
        assert_eq!(foobar(3), Some("Foo".to_string()));
    }

    #[test]
    fn returns_bar_when_dividable_by_5() {
        assert_eq!(foobar(5), Some("Bar".to_string()));
    }

    #[test]
    fn returns_foobar_when_dividable_by_5_and_3() {
        assert_eq!(foobar(45), Some("FooBar".to_string()));
    }

    #[test]
    fn returns_nothing_when_not_dividable_by_5_nor_3() {
        assert_eq!(foobar(67), None);
    }
}