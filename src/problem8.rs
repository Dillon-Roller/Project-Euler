#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_problem7() {
        assert_eq!(solve_problem7(), 104743)
    }

    fn solve_problem7() -> u32 {
        let num_str = fs::read_to_string("number.txt")
            .expect("Something went wrong reading the file")
            .replace("\n", ""); 

        num_str
            .chars()
            .collect::<Vec<char>>() 
            .windows(13) 
            .map(|window| {
                window.iter()
                .filter_map(|c| c.to_digit(10)) 
                .product::<u32>() 
            })
            .max() 
            .unwrap()
    }
}
