#[cfg(test)]
mod tests {
    #[test]
    fn test_problem4() {
        assert!(is_palindrome(1001));
        assert!(!is_palindrome(1002));

        assert_eq!(solve_problem4(), 997799);
    }

    fn solve_problem4() -> u64 {
        (2..999*999)
            .filter(|&x| is_palindrome(x))
            .last()
            .unwrap()
    }

    fn is_palindrome(n: u64) -> bool {
        let s = n.to_string()
            .chars()
            .collect::<Vec<char>>();

        s == s.iter()
            .rev()
            .cloned()
            .collect::<Vec<char>>()
    }
}
