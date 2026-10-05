//! Computes the number of steps it takes to reach 1 for a given number,
//! following the Collatz conjecture's rules.

/// Computes the number of steps required to reach `1` from `n` using the
/// Collatz sequence rules: halve even numbers, and triple-plus-one odd
/// numbers.
///
/// Returns `None` for `n == 0`, since `0` is not a valid starting point for
/// the sequence. Otherwise returns `Some(steps)`.
///
/// # Examples
///
/// ```
/// use collatz_conjecture::collatz;
///
/// assert_eq!(collatz(1), Some(0));
/// assert_eq!(collatz(16), Some(4));
/// assert_eq!(collatz(0), None);
/// ```
pub fn collatz(n: u64) -> Option<u64> {
    let mut a = n;
    let mut steps: u64 = 0;
    if a == 0 {
        return None;
    }

    while a != 1 {
        if a.is_multiple_of(2) {
            a /= 2;
        } else {
            a = (a * 3) + 1;
        }
        steps += 1;
    }
    Some(steps)
}
