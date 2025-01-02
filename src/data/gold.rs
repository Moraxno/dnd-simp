use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GoldAmount {
    copper: isize,
}

pub type Gold = isize;
pub type Silver = isize;
pub type Copper = isize;

impl GoldAmount {
    pub fn new(gold: isize, silver: isize, copper: isize) -> Self {
        let gold_copper = gold.saturating_mul(100);
        let silver_copper = gold.saturating_mul(10);

        let all_copper = gold_copper
            .saturating_add(silver_copper)
            .saturating_add(copper);

        Self {
            copper: all_copper
        }
    }

    pub fn gold(&self) -> Gold {
        self.copper.signum() * self.copper.abs() / 100
    }

    pub fn silver(&self) -> Silver {
        self.copper.signum() * ((self.copper.abs() % 100) / 10)
    }

    pub fn copper(&self) -> Copper {
        self.copper.signum() * ((self.copper.abs() % 10) / 10)
    }

    pub fn as_tuple(&self) -> (Gold, Silver, Copper) {
        (self.gold(), self.silver(), self.copper())
    }

    pub fn as_gold(&self) -> f32 {
        self.copper as f32 / 100.0f32
    }

    pub fn as_silver(&self) -> f32 {
        self.copper as f32 / 10.0f32
    }

    pub fn as_copper(&self) -> f32 {
        self.copper as f32
    }

    pub fn as_electrum(&self) -> f32 {
        self.copper as f32 / 50.0f32
    }

    pub fn gold_str(&self) -> String {
        format!("{} gp", self.gold())
    }

    pub fn silver_str(&self) -> String {
        format!("{} sp", self.silver())
    }

    pub fn copper_str(&self) -> String {
        format!("{} cp", self.copper())
    }
}

impl ToString for GoldAmount {
    fn to_string(&self) -> String {
        let amount_pattern = self.as_tuple();

        log::info!("Gold Pattern {:?}", amount_pattern);

        match amount_pattern {
            (0, 0, 0) => self.gold_str(),
            (0, 0, _) => self.copper_str(),
            (0, _, 0) => self.silver_str(),
            (_, 0, 0) => self.gold_str(),
            (0, _, _) => format!("{} {}", self.silver_str(), self.copper_str()),
            (_, _, 0) => format!("{} {}",  self.gold_str(), self.silver_str()),
            (_, _, _) => format!("{}, {} {}", self.gold_str(), self.silver_str(), self.copper_str()),
        }
    }
}

impl Add for GoldAmount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            copper: self.copper + rhs.copper
        }
    }
}

impl Mul<isize> for GoldAmount {
    type Output = Self;
    
    fn mul(self, rhs: isize) -> Self::Output {
        Self::Output {
            copper: self.copper * rhs
        }
    }
}

impl Neg for GoldAmount {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        self * -1
    }
}

impl Sub for GoldAmount {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(-rhs)
    }
}