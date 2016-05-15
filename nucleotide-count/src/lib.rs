use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> usize {
    dna.matches(nucleotide).count()
}

pub fn nucleotide_counts(dna: &str) -> HashMap<char, usize> {
    "ACGT".chars().map(|c| (c, count(c, dna))).collect()
}
