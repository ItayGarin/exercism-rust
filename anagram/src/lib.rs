use std::string::String;

fn sort(s: &String) -> String {
    let mut sorted: Vec<char> = s.chars().collect();
    sorted.sort();
    let mut result = String::with_capacity(s.len());
    for c in sorted {
        result.push(c);
    }
    result
}

pub fn anagrams_for<'a>(string: &str, candidates: &[&'a str]) -> Vec<&'a str> {
    let lower = string.to_lowercase();
    let sorted = sort(&lower);

    let mut anagrams = Vec::new();
    for candidate in candidates.iter() {
        let lower_candidate = candidate.to_lowercase();
        if lower != lower_candidate {
            if sorted == sort(&lower_candidate) {
                anagrams.push(*candidate)
            }
        }
    }

    anagrams
}
