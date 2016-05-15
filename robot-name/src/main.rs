extern crate rand;

fn rand_capital_char() -> char {
    let mut rand = rand::random::<u8>();
    rand %= 26;
    rand += 'A' as u8;
    rand as char
}

fn rand_three_digits_num() -> u16 {
    let mut rand = rand::random::<u16>();
    rand %= 1000;
    rand
}


fn main() {
    let name = format!("{}{}{:03}",
                       rand_capital_char(),
                       rand_capital_char(),
                       rand_three_digits_num());
    println!("name = {}", name);
}
