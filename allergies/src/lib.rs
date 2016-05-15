pub struct Allergies {
    score: u32,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

fn allergens() -> Vec<Allergen> {
    vec![Allergen::Eggs,
         Allergen::Peanuts,
         Allergen::Shellfish,
         Allergen::Strawberries,
         Allergen::Tomatoes,
         Allergen::Chocolate,
         Allergen::Pollen,
         Allergen::Cats]
}

impl Allergies {
    pub fn new(score: u32) -> Allergies {
        Allergies{score: score}
    }
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let int_allergen = *allergen as u32;
        return (self.score & int_allergen) == int_allergen
    }
    pub fn allergies(&self) -> Vec<Allergen> {
        allergens()
            .into_iter()
            .filter(|allergen| self.is_allergic_to(allergen))
            .collect()
    }
}
