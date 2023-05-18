#[cfg(test)]
mod tests {
    pub struct Fibonacci {
        curr: u32,
        next: u32,
    }
    
    impl Fibonacci {
        pub fn new() -> Fibonacci {
            Fibonacci { curr: 0, next: 1 }
        }
    }
    
    impl Iterator for Fibonacci {
        type Item = u32;
    
        fn next(&mut self) -> Option<u32> {
            let new_next = self.curr + self.next;
            self.curr = self.next;
            self.next = new_next;
    
            Some(self.curr)
        }
    }
    
    #[test]
    fn test_problem2() {
        assert_eq!(solve_problem2(), 4_613_732);  
    }

    fn solve_problem2() -> u32 {
        Fibonacci::new()
            .take_while(|&x| x < 4_000_000)
            .filter(|&x| x % 2 == 0)
            .sum()
    }
}
