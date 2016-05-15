use std::collections::HashMap;
use std::char;

fn words(phrase: &str) -> Vec<String> {
    phrase.split(|c| !char::is_alphanumeric(c))
        .filter(|s| s.len() != 0)
        .map(|s| s.to_lowercase())
        .collect()
}

pub fn word_count(phrase: &str) -> HashMap<String, u32> {
    let words = words(phrase);

    let mut count_map = HashMap::new();
    for word in words {
        *count_map.entry(word).or_insert(0) += 1;
    }

    count_map
}
