#[cfg(test)]
mod tests {
    
    #[test]
    fn test_problem9() {
        assert_eq!(solve_problem9(), 4_613_732);  
    }

    fn solve_problem9() -> u64 {
        find_pythagorean_triplet(1000)
    }

    fn find_pythagorean_triplet(sum: u64) -> u64 {
        (1..=sum / 2)
            .find_map(|m| {
                let n = sum / 2 - m;
                if m > n && m * (m + n) == sum * sum / 2 {
                    Some((m * m - n * n) * (2 * m * n) * (m * m + n * n)) 
                } else {
                    None
                }
            })
            .unwrap()
    }
}
