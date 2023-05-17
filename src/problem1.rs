fn solve_problem1() -> i32 {
    (2..1000)
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        assert_eq!(solve_problem1(), 233168);  // replace this with your function and expected result
    }
}
