use std::{
    fmt::Display,
    ops::{Add, Mul, Neg, Sub},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GoldAmount {
    copper: isize,
}

pub type Gold = isize;
pub type Silver = isize;
pub type Copper = isize;

impl GoldAmount {
    pub fn from_copper(copper: isize) -> Self {
        Self { copper }
    }

    pub fn from_silver(silver: isize) -> Self {
        Self {
            copper: silver * 10,
        }
    }

    pub fn from_electrum(electrum: isize) -> Self {
        Self {
            copper: electrum * 50,
        }
    }

    pub fn from_gold(gold: isize) -> Self {
        Self { copper: gold * 100 }
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

pub trait AsGoldCurrency {
    fn gold(&self) -> GoldAmount;
    fn silver(&self) -> GoldAmount;
    fn copper(&self) -> GoldAmount;
}

impl AsGoldCurrency for isize {
    fn gold(&self) -> GoldAmount {
        GoldAmount::from_gold(*self)
    }

    fn silver(&self) -> GoldAmount {
        GoldAmount::from_silver(*self)
    }

    fn copper(&self) -> GoldAmount {
        GoldAmount::from_copper(*self)
    }
}

impl Display for GoldAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let amount_pattern = self.as_tuple();
        let s = match amount_pattern {
            (0, 0, 0) => self.gold_str(),
            (0, 0, _) => self.copper_str(),
            (0, _, 0) => self.silver_str(),
            (_, 0, 0) => self.gold_str(),
            (0, _, _) => format!("{} {}", self.silver_str(), self.copper_str()),
            (_, _, 0) => format!("{} {}", self.gold_str(), self.silver_str()),
            (_, _, _) => format!(
                "{}, {} {}",
                self.gold_str(),
                self.silver_str(),
                self.copper_str()
            ),
        };

        f.write_str(&s)
    }
}

impl Add for GoldAmount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            copper: self.copper + rhs.copper,
        }
    }
}

impl Mul<isize> for GoldAmount {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self::Output {
            copper: self.copper * rhs,
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
