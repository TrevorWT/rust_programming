//! Bob is a lackadaisical teenager who answers questions, exclamations,
//! silence, and everything else with a small, fixed set of replies.

/// Returns Bob's reply to the given `message`.
///
/// Bob only cares about three things: whether the message is a question
/// (ends with `?`), whether it is shouted (has letters and no lowercase
/// ones), and whether it is silence (empty after trimming whitespace).
///
/// # Examples
///
/// ```
/// use bob::reply;
///
/// assert_eq!(reply("How are you?"), "Sure.");
/// assert_eq!(reply("WATCH OUT!"), "Whoa, chill out!");
/// assert_eq!(reply(""), "Fine. Be that way!");
/// ```
pub fn reply(message: &str) -> &str {
    let mut a = 0;
    if message.trim().is_empty() {
        a += 5;
    }
    if !message.chars().any(char::is_lowercase) && message.chars().any(char::is_alphabetic) {
        a += 1;
    }
    if message.trim().ends_with('?') {
        a += 2;
    }
    match a {
        5 => "Fine. Be that way!",
        1 => "Whoa, chill out!",
        2 => "Sure.",
        3 => "Calm down, I know what I'm doing!",
        _ => "Whatever.",
    }
}
