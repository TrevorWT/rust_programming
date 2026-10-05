//! Computes the prime factorization of a positive integer.

/// Returns the prime factors of `n` in ascending order, with repeated
/// factors listed multiple times (e.g. `factors(8) == [2, 2, 2]`).
///
/// Returns an empty vector for `n < 2`, since such numbers have no prime
/// factors.
///
/// # Examples
///
/// ```
/// use prime_factors::factors;
///
/// assert_eq!(factors(1), Vec::<u64>::new());
/// assert_eq!(factors(12), vec![2, 2, 3]);
/// ```
pub fn factors(n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    if n < 2 {
        return result;
    }

    let mut remaining = n;
    let mut candidate = 2;
    while candidate <= remaining {
        if remaining.is_multiple_of(candidate) {
            remaining /= candidate;
            result.push(candidate);
        } else {
            candidate += 1;
        }
    }
    result
}
