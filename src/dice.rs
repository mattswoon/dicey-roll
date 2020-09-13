use rand::{
    RngCore,
    distributions::{Distribution, Uniform}
};

pub enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20
}

impl Dice {
    pub fn roll<R: RngCore>(&self, mut rng: R) -> u8 {
        match self {
            Dice::D4 => Uniform::new_inclusive(1, 4).sample(&mut rng),
            Dice::D6 => Uniform::new_inclusive(1, 6).sample(&mut rng),
            Dice::D8 => Uniform::new_inclusive(1, 8).sample(&mut rng),
            Dice::D10 => Uniform::new_inclusive(1, 10).sample(&mut rng),
            Dice::D12 => Uniform::new_inclusive(1, 12).sample(&mut rng),
            Dice::D20 => Uniform::new_inclusive(1, 20).sample(&mut rng),
        }
    }
}
