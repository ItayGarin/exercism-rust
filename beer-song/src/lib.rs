fn capitilize(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                c.to_uppercase().next().unwrap()
            } else {
                c
            }
        })
        .collect()
}

fn next_bottle_count(n: u8) -> u8 {
    match n {
        0 => 99,
        n @ _ => n - 1,
    }
}

fn n_to_bottles(n: u8) -> String {
    match n {
        0 => "no more bottles".to_string(),
        1 => "1 bottle".to_string(),
        n @ _ => format!("{} bottles", n),
    }
}

fn n_to_instruction(n: u8) -> &'static str {
    match n {
        0 => "Go to the store and buy some more",
        1 => "Take it down and pass it around",
        _ => "Take one down and pass it around",
    }
}

pub fn verse(n: u8) -> String {
    let current = n_to_bottles(n);
    let next = n_to_bottles(next_bottle_count(n));
    let instruction = n_to_instruction(n);
    format!("{} of beer on the wall, {} of beer.\n{}, {} of beer on the wall.\n",
            capitilize(&current), current, instruction, next)
}

pub fn sing(start: u8, end: u8) -> String {
    let bottles = end .. start+1 as u8;
    let verses: Vec<String> = bottles
        .rev()
        .map(|n| verse(n))
        .collect();
    verses.join("\n")
}
