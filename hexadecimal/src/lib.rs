fn parse_hex_digit(c: char) -> Option<u32> {
    match c {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        'a' => Some(10),
        'b' => Some(11),
        'c' => Some(12),
        'd' => Some(13),
        'e' => Some(14),
        'f' => Some(15),
        _   => None,
    }
}

pub fn hex_to_int(input: &str) -> Option<u32> {
    let lower = input.to_lowercase();
    let positions = 0 .. input.len() as u32;
    let base: u32 = 16;

    lower
        .chars()
        .rev()
        .zip(positions)
        .fold(Some(0), |acc, (c, pos)| {
            parse_hex_digit(c).and_then(|n| {
                acc.map(|acc| acc + n * base.pow(pos))
            })
        })
}
