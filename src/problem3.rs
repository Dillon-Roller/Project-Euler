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
    }

    fn largest_prime_factor(mut n: u64) -> u64 {
        let mut p = 2;
        loop {
            p = smallest_prime_factor(p, n);
            if n != p {
                n /= p;
            }
            else {
                return n;
            }
        }
    }

    fn smallest_prime_factor(start: u64, n: u64) -> u64{
        (start..=n.sqrt() + 1)
            .find(|&x| n % x == 0)
            .unwrap_or(n)
    }
}
