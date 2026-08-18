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
    pub fn new(score: u32) -> Self {
        Self {
            bin_score: format!("{:b}", score % 256).chars().rev().collect(),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        if let Some(x) = self.bin_score.get(*allergen as usize) {
            return *x == '1';
        }

        false
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        vec![
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
        .map(|a| a.clone())
        .collect()
    }
}
