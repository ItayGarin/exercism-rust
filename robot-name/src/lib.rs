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

fn generate_random_name() -> String {
    format!("{}{}{:03}",
            rand_capital_char(),
            rand_capital_char(),
            rand_three_digits_num())
}

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Robot {
        Robot{name: generate_random_name()}
    }

    pub fn name<'a>(&'a self) -> &'a str {
        &self.name
    }

    pub fn reset_name(&mut self) {
       self.name = generate_random_name(); 
    }
}
