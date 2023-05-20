#[cfg(test)]
mod tests {
    #[test]
    fn test_problem4() {
        assert!(is_palindrome(1001));
        assert!(!is_palindrome(1002));

        assert_eq!(solve_problem4(), 906609);
    }

    fn solve_problem4() -> u64 {
        (100..1000)
            .flat_map(|i| {
                (i..1000).map(move |j| i * j)
            })
            .filter(|&n| is_palindrome(n))
            .max()
            .unwrap()
    }
    
    fn is_palindrome(n: u64) -> bool {
        n.to_string().chars().collect::<String>() == n.to_string().chars().rev().collect::<String>()
    }
}
