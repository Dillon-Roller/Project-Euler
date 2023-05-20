#[cfg(test)]
mod tests {
    #[test]
    fn test_problem5() {
        assert_eq!(solve_problem5(), 232792560);
    }

    fn solve_problem5() -> u64 {
        (1..=20).fold(1, lcm)
    }
    
    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 { a } else { gcd(b, a % b) }
    }
    
    fn lcm(a: u64, b: u64) -> u64 {
        a * b / gcd(a, b)
    }
    
}
