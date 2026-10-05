//! Computes the difference between the square of the sum and the sum of the
//! squares of the first `n` natural numbers.

/// Returns the square of the sum of the first `n` natural numbers:
/// `(1 + 2 + ... + n)^2`.
///
/// # Examples
///
/// ```
/// use difference_of_squares::square_of_sum;
///
/// assert_eq!(square_of_sum(5), 225);
/// ```
pub fn square_of_sum(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum.pow(2)
}

/// Returns the sum of the squares of the first `n` natural numbers:
/// `1^2 + 2^2 + ... + n^2`.
///
/// # Examples
///
/// ```
/// use difference_of_squares::sum_of_squares;
///
/// assert_eq!(sum_of_squares(5), 55);
/// ```
pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i.pow(2);
    }
    sum
}

/// Returns the difference between [`square_of_sum`] and [`sum_of_squares`]
/// for the first `n` natural numbers.
///
/// # Examples
///
/// ```
/// use difference_of_squares::difference;
///
/// assert_eq!(difference(5), 170);
/// ```
pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
