#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_problem8() {
        assert_eq!(solve_problem8(), 23514624000)
    }

    fn solve_problem8() -> u64 {
        fs::read_to_string("input/problem8_input.txt")
            .expect("input file should exist")
            .replace("\n", "")
            .chars()
            .collect::<Vec<char>>() 
            .windows(13) 
            .map(|window| {
                window.iter()
                    .filter_map(|c| c.to_digit(10)) 
                    .map(|n| n as u64)
                    .product::<u64>() 
            })
            .max() 
            .unwrap()
    }
}
