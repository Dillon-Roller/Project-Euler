#[cfg(test)]
mod tests {
    use num_integer::Roots;

    #[test]
    fn test_problem3() {
        assert_eq!(smallest_prime_factor(2, 2), 2);
        assert_eq!(smallest_prime_factor(2, 3), 3);
        assert_eq!(smallest_prime_factor(2, 4), 2);
        assert_eq!(smallest_prime_factor(2, 4), 2);

        assert_eq!(largest_prime_factor(21), 7);

        assert_eq!(solve_problem3(), 0)
    }

    fn solve_problem3() -> u64 {
        largest_prime_factor(600_851_475_143)
    }

    fn largest_prime_factor(mut n: u64) -> u64 {
        let mut p = 2;
        while p < n {
            p = smallest_prime_factor(p, n);
            if n != p {
                n /= p;
            }
        }
        n
    }

    fn smallest_prime_factor(start: u64, n: u64) -> u64 {
        (start..=n.sqrt() + 1)
            .find(|&x| n % x == 0)
            .unwrap_or(n)
    }
}
