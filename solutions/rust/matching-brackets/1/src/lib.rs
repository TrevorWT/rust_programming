pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in string.chars() {
        match c {
            '(' => stack.push('('),
            '[' => stack.push('['),
            '{' => stack.push('{'),
            ')' => { if stack.last() == Some(&'(') { stack.pop(); } else { return false; }},
            ']' => { if stack.last() == Some(&'[') { stack.pop(); } else { return false; }},
            '}' => { if stack.last() == Some(&'{') { stack.pop(); } else { return false; }},
             _ => {} 
        }
    }
    if !stack.is_empty() { return false; }
    true
}
