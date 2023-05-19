#[cfg(test)]
mod tests {
    #[test]
    fn test_problem4() {
        
    }

    fn is_palindrome(n: u64) -> bool {
        let s = n.to_string()
            .chars()
            .collect::<Vec<char>>();

        s == s.iter()
            .rev()
            .cloned()
            .collect::<Vec<char>>()
    }
}
