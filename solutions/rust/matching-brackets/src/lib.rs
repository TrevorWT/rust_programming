//! Determines whether the brackets in a string are balanced and correctly
//! nested, ignoring any non-bracket characters.

/// Returns `true` if every `(`, `[`, and `{` in `string` has a matching,
/// correctly nested closing bracket, and `false` otherwise.
///
/// Characters other than brackets are ignored.
///
/// # Examples
///
/// ```
/// use matching_brackets::brackets_are_balanced;
///
/// assert!(brackets_are_balanced("{[]}"));
/// assert!(!brackets_are_balanced("{[}]"));
/// ```
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in string.chars() {
        match c {
            '(' => stack.push('('),
            '[' => stack.push('['),
            '{' => stack.push('{'),
            ')' => {
                if stack.last() == Some(&'(') {
                    stack.pop();
                } else {
                    return false;
                }
            }
            ']' => {
                if stack.last() == Some(&'[') {
                    stack.pop();
                } else {
                    return false;
                }
            }
            '}' => {
                if stack.last() == Some(&'{') {
                    stack.pop();
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}
