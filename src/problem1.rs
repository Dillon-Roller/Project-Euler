#[cfg(test)]
mod tests {
    #[test]
    fn test_problem1() {
        assert_eq!(solve_problem1(), 233168);  
    }

    fn solve_problem1() -> i32 {
        (2..1000)
            .filter(|x| x % 3 == 0 || x % 5 == 0)
            .sum()
    }
}
