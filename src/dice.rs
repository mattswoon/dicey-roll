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

impl std::fmt::Display for Dice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Dice::D4 => write!(f, "d4"),
            Dice::D6 => write!(f, "d6"),
            Dice::D8 => write!(f, "d8"),
            Dice::D10 => write!(f, "d10"),
            Dice::D12 => write!(f, "d12"),
            Dice::D20 => write!(f, "d20")
        }
    }
}

pub struct DiceTuple {
    pub count: u32,
    pub dice: Dice
}

impl std::str::FromStr for DiceTuple {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("d");
        let num: u32 = split.next()
            .ok_or(Error::ParseError(format!("Invalid dice string: {}", s)))?
            .parse()
            .unwrap_or(1);
        let dice = match split.next() {
            Some("4") => Ok(Dice::D4),
            Some("6") => Ok(Dice::D6),
            Some("8") => Ok(Dice::D8),
            Some("10") => Ok(Dice::D10),
            Some("12") => Ok(Dice::D12),
            Some("20") => Ok(Dice::D20),
            Some(d) => Err(Error::ParseError(format!("Invalid dice: {}", d))),
            None => Err(Error::ParseError(format!("Invalid dice string: {}", s))),
        }?;
        Ok(DiceTuple { count: num, dice: dice })
    }
}

#[derive(Debug)]
pub enum Error {
    ParseError(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        dbg!(self);
        match self {
            Error::ParseError(s) => write!(f, "{}", s),
        }
    }
}
