use std::char;

fn filter_numbers(input: &str) -> String {
    input.chars()
        .filter(|&c| char::is_numeric(c))
        .collect()
}

fn remove_first_char(input: String) -> String {
    let mut output = input;
    output.remove(0);
    output
}

fn trim_prefix(input: String) -> String {
    if input.len() == 11 && input.starts_with('1') {
        remove_first_char(input)
    } else {
        input
    }
}

pub fn number(input: &str) -> Option<String> {
    let numbers = filter_numbers(input);
    let numbers = trim_prefix(numbers);

    match numbers.len() {
        10 => Some(numbers),
        _ => None,
    }
}

pub fn area_code(input: &str) -> Option<String> {
    number(input).map(|n| n[..3].to_string())
}

fn _pretty_print(input: String) -> String {
    let area = &input[..3];
    let middle = &input[3..6];
    let last = &input[6..];
    format!("({}) {}-{}", area, middle, last)
}

pub fn pretty_print(input: &str) -> String {
    let number = number(input);
    match number {
        Some(n) => _pretty_print(n),
        None => "invalid".to_string(),
    }
}
