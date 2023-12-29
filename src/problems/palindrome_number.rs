pub fn is_palindrome(x: i32) -> bool {
    let s = x.to_string();

    s == s.chars().rev().collect::<String>()
}

#[cfg(test)]
mod test_palindrome_number {
    use crate::problems::palindrome_number::is_palindrome;

    #[test]
    fn test1() {
        assert!(is_palindrome(121));
    }

    #[test]
    fn test2() {
        assert!(is_palindrome(-121));
    }

    #[test]
    fn test3() {
        assert!(is_palindrome(101));
    }
}