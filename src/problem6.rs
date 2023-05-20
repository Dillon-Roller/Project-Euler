#[cfg(test)]
mod tests {
    #[test]
    fn test_problem6() {
        assert_eq!(solve_problem6(), 232792560);
    }

    fn solve_problem6() -> u64 {
        (1..=20).fold(1, lcm)
    }
}
