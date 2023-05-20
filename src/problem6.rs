#[cfg(test)]
mod tests {
    #[test]
    fn test_problem6() {
        assert_eq!(solve_problem6(), 25164150);
    }

    fn solve_problem6() -> u64 {
        let n = 100;
    
        let sum_of_squares: u64 = (1..=n).map(|x| x * x).sum();
        let square_of_sum: u64 = (1..=n).sum::<u64>().pow(2);
    
        square_of_sum - sum_of_squares
    }
}
