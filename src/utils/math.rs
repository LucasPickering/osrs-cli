/// Calculate numerical combination `nCk` (n choose k).
pub fn combination(n: usize, k: usize) -> usize {
    // safety check to prevent overflow/underflow
    if k > n {
        return 0;
    }

    // This is a reduction of the formula `n! / ((n - k)! * k!)`. I've cut it
    // down to:
    // ((n - k + 1) * (n - k + 2) * ... * (n - 1) * n) / k!
    // This reduces the max values we reach, so we stay in bounds for usize
    let num: usize = ((n - k + 1)..=n).product();
    let denom: usize = (1..=k).product();
    num / denom
}

/// Calculate the binomial distribution. This calculates the odds of getting
/// **exactly** `k` successes in `n` trials, where each trial has `p`
/// probability of success.
// https://en.wikipedia.org/wiki/Binomial_distribution
pub fn binomial(p: f64, n: usize, k: usize) -> f64 {
    // Validate inputs
    if !(0.0..=1.0).contains(&p) {
        panic!("Probability must be in [0, 1], got: {}", p);
    }
    if k > n {
        panic!(
            "n (# of trials) must be >= k (# of successes), but got n={}, k={}",
            n, k
        );
    }

    p.powi(k as i32) * (1.0 - p).powi((n - k) as i32) * combination(n, k) as f64
}

/// The Cumulative Distribution Function for a binomial distrobution, i.e.
/// calculate a binomial probability over a range of success values rather than
/// a single one. Calculates the odds of getting `k_x` successes in `n` trials,
/// where `k_x` is any value in `k_values` and each trial has `p` probability of
/// success.
pub fn binomial_cdf(
    p: f64,
    n: usize,
    k_values: &mut dyn Iterator<Item = usize>,
) -> f64 {
    k_values.map(|k_i| binomial(p, n, k_i)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_combination() {
        assert_eq!(combination(0, 0), 1);
        assert_eq!(combination(0, 1), 0);
        assert_eq!(combination(1, 0), 1);
        assert_eq!(combination(1, 1), 1);

        assert_eq!(combination(2, 0), 1);
        assert_eq!(combination(2, 1), 2);
        assert_eq!(combination(2, 2), 1);

        assert_eq!(combination(5, 0), 1);
        assert_eq!(combination(5, 1), 5);
        assert_eq!(combination(5, 2), 10);
        assert_eq!(combination(5, 3), 10);
        assert_eq!(combination(5, 4), 5);
        assert_eq!(combination(5, 5), 1);

        assert_eq!(combination(10, 5), 252);
    }

    #[test]
    fn test_binomial() {
        assert_approx_eq!(binomial(0.5, 2, 1), 0.5);
        assert_approx_eq!(binomial(0.1, 20, 0), 0.121577);
        assert_approx_eq!(binomial(0.1, 20, 6), 0.00886704);
    }

    #[test]
    #[should_panic]
    fn test_binomial_invalid_prob() {
        binomial(1.1, 2, 1);
    }

    #[test]
    #[should_panic]
    fn test_binomial_k_gt_n() {
        binomial(0.5, 1, 2);
    }

    #[test]
    fn test_binomial_cdf() {
        assert_approx_eq!(binomial_cdf(0.5, 2, &mut (0..=0)), 0.25);
        assert_approx_eq!(binomial_cdf(0.5, 2, &mut (0..=1)), 0.75);
        assert_approx_eq!(binomial_cdf(0.5, 2, &mut (0..=2)), 1.0);
    }
}
