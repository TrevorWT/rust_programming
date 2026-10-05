//! Reverses a string's characters.

/// Returns `input` with its characters in reverse order.
///
/// This operates on Unicode scalar values (`char`s), not grapheme clusters,
/// so combining characters are not specially handled.
///
/// # Examples
///
/// ```
/// use reverse_string::reverse;
///
/// assert_eq!(reverse("robot"), "tobor");
/// ```
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
