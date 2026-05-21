use std::{fmt, str::FromStr};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl Antigen {
    pub const EVERY: &[Self] = &[Self::AB, Self::O, Self::A, Self::B];
}

impl RhFactor {
    pub const EVERY: &[Self] = &[Self::Positive, Self::Negative];
}

impl FromStr for Antigen {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "AB" => Ok(Self::AB),
            "O" => Ok(Self::O),
            _ => Err(()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Positive),
            "-" => Ok(Self::Negative),
            _ => Err(()),
        }
    }
}

impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(());
        }

        let antigen_part = &s[..s.len() - 1];
        let rh_part = &s[s.len() - 1..];

        Ok(Self {
            antigen: antigen_part.parse()?,
            rh_factor: rh_part.parse()?,
        })
    }
}

impl fmt::Debug for Antigen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::A => "A",
            Self::AB => "AB",
            Self::B => "B",
            Self::O => "O",
        };

        write!(f, "{text}")
    }
}

impl fmt::Debug for RhFactor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Positive => "+",
            Self::Negative => "-",
        };

        write!(f, "{text}")
    }
}

impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{:?}", self.antigen, self.rh_factor)
    }
}

impl BloodType {
    pub fn every() -> impl Iterator<Item = Self> {
        Antigen::EVERY.iter().copied().flat_map(|antigen| {
            RhFactor::EVERY.iter().copied().map(move |rh_factor| Self {
                antigen,
                rh_factor,
            })
        })
    }

    pub fn can_receive_from(self, other: Self) -> bool {
        let rh_ok =
            self.rh_factor == RhFactor::Positive || other.rh_factor == RhFactor::Negative;

        let antigen_ok =
            other.antigen == Antigen::O ||
            self.antigen == Antigen::AB ||
            self.antigen == other.antigen;

        rh_ok && antigen_ok
    }

    pub fn donors(self) -> Vec<Self> {
        Self::every()
            .filter(|&blood| self.can_receive_from(blood))
            .collect()
    }

    pub fn recipients(self) -> Vec<Self> {
        Self::every()
            .filter(|&blood| blood.can_receive_from(self))
            .collect()
    }
}