pub fn hamming_distance(a: &str, b: &str) -> Result<u32, &'static str> {
    if a.len() != b.len() {
        return Err("inputs of different length");
    }

    let char_a = a.chars();
    let char_b = b.chars();
    let distance = char_a.zip(char_b)
        .fold(0,
              |accum, (a, b)|
              if a != b {
                  accum + 1
              } else {
                  accum
              });
    Ok(distance)
}
