pub struct Allergies {
    bin_score: Vec<char>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    // Since the allergens map to powers of 2, we can just check against the binary version of the number
    // Allergies has bin_score a vector of 0s and 1s
    pub fn new(score: u32) -> Self {
        Self {
            bin_score: format!("{:b}", score % 256).chars().rev().collect(),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        // Check if 2 ^ allergen == 1 or 0
        if let Some(x) = self.bin_score.get(*allergen as usize) {
            return *x == '1';
        }

        false
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        [
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ]
        .iter()
        .filter(|x| self.is_allergic_to(x))
        .copied()
        .collect()
    }
}
