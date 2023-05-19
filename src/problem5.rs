#[cfg(test)]
mod tests {
    #[test]
    fn test_problem4() {
        assert_eq!(solve_problem4(), 232792560);
    }

    fn solve_problem4() -> u64 {
        (1..=20).fold(1, lcm)
    }
    
    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 { a } else { gcd(b, a % b) }
    }
    
    fn lcm(a: u64, b: u64) -> u64 {
        a * b / gcd(a, b)
    }
    
}
