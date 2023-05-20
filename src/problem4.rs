#[cfg(test)]
mod tests {

    #[test]
    fn test_problem4() {
        assert_eq!(solve_problem4(), 906609);
    }

    fn solve_problem4() -> i32 {
        (100..1000).rev()
            .map(|num| {
                let palindrome = num.to_string() + &num.to_string().chars().rev().collect::<String>();
                palindrome.parse::<i32>().unwrap()
            })
            .find(|&n| can_be_factored(n))
            .unwrap()
    }
    
    fn can_be_factored(n: i32) -> bool {
        (100..1000)
            .any(|i| n % i == 0 && (100..=999).contains(&(n / i)))
    }
    
}   