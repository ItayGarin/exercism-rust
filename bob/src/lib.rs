use std::char;

fn is_yelling(input: &str) -> bool {
    input.chars().all(|c| !char::is_lowercase(c))
}

fn is_question(input: &str) -> bool {
    input.ends_with("?")
}

pub fn reply(input: &str) -> &str {
    if input.is_empty() { "Fine. Be that way!" }
    else if is_question(input) { "Sure." }
    else if is_yelling(input) { "Whoa, chill out!"}
    else { "Whatever." }
}
