pub fn reply(message: &str) -> &str {
    let mut a = 0;
    if message.trim().is_empty() { a += 5; }
    if !message.chars().any(char::is_lowercase) && message.chars().any(char::is_alphabetic) {a+=1; }
    if message.trim().ends_with('?') { a+=2; }
    match a {
        5 => "Fine. Be that way!",
        1 => "Whoa, chill out!",
        2 => "Sure.",
        3 => "Calm down, I know what I'm doing!",
        _ => "Whatever."
    }
    
}
