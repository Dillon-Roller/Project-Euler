#[cfg(test)]
mod tests {
    #[test]
    fn test_problem4() {
        
    }

    fn is_palindrome(n: u64) -> bool {
        let n_str = n.to_string()
            .chars()
            .collect::<Vec<char>>();

        n_str == n_str.iter()
            .rev()
            .cloned()
            .collect::<Vec<char>>()
    }
}
