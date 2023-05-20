#[cfg(test)]
mod tests {
    use rayon::prelude::*;
    use num_integer::Roots;

    const CHUNK_SIZE: usize = 1_000_000;
    const NTH_PRIME: usize = 10_001;

    #[test]
    fn test_problem7() {
        assert_eq!(solve_problem7(), 104743)
    }

    fn solve_problem7() -> usize {
        let mut chunk_start = 0;
        let mut primes = vec![];

        while primes.len() < NTH_PRIME {
            primes.extend(
                (chunk_start..chunk_start + CHUNK_SIZE).into_par_iter()
                .filter(|&n| n > 1 && (2..=n.sqrt()).all(|i| n % i != 0))
                .collect::<Vec<_>>()
            );

            chunk_start += CHUNK_SIZE;
        }
        
        primes[NTH_PRIME - 1]
    }
}
