use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum QuartType {
    PhiJ,
    PhiK,
    PhiJK,
    PhiKJ,
    phij,
    phijk,
    phik,
}

impl FromStr for QuartType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use QuartType::*;
        match s {
            "PHIJ" => Ok(PhiJ),
            "PHIK" => Ok(PhiK),
            "PHIJK" => Ok(PhiJK),
            "PHIKJ" => Ok(PhiKJ),
            "phij" => Ok(phij),
            "phijk" => Ok(phijk),
            "phik" => Ok(phik),
            _ => Err(()),
        }
    }
}

impl Display for QuartType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use QuartType::*;
        f.pad(match self {
            PhiJ => "PHIJ",
            PhiK => "PHIK",
            PhiJK => "PHIJK",
            PhiKJ => "PHIKJ",
            phij => "phij",
            phijk => "phijk",
            phik => "phik",
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Phi {
    pub(crate) typ: QuartType,
    pub(crate) val: f64,
}

impl Phi {
    pub fn new(typ: &str, val: f64) -> Self {
        Self {
            typ: typ.parse().unwrap(),
            val,
        }
    }
}
