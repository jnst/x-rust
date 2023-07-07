pub fn is_palindrome1(x: i32) -> bool {
    let s = x.to_string();
    s.chars().rev().collect::<String>() == s
}

pub fn is_palindrome2(x: i32) -> bool {
    let s = x.to_string();
    let len = s.len();
    let v = s.chars().collect::<Vec<char>>();
    for i in 0..len / 2 {
        if v[i] != v[len - 1 - i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome1() {
        assert!(is_palindrome1(121));
        assert!(!is_palindrome1(-121));
        assert!(!is_palindrome1(10));
    }

    #[test]
    fn test_is_palindrome2() {
        assert!(is_palindrome2(121));
        assert!(!is_palindrome2(-121));
        assert!(!is_palindrome2(10));
    }
}
