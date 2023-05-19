#[cfg(test)]
mod tests {
    #[test]
    fn test_problem4() {
        assert!(is_palindrome(1001));
        assert!(!is_palindrome(1002));

        assert_eq!(solve_problem4(), 997799);
    }

    fn solve_problem4() -> u64 {
        (100*100..999*999)
            .rev()
            .find(|&n| is_palindrome(n))
            .unwrap()
    }
    
    fn is_palindrome(n: u64) -> bool {
        let s = n.to_string();
        
        s == s.chars()
            .rev()
            .collect::<String>()
    }
}
