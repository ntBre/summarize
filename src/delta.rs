use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum QuartType {
    DeltaJ,
    DeltaK,
    DeltaJK,
    deltaJ,
    deltaK,
}

impl FromStr for QuartType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use QuartType::*;
        match s {
            "DELTAJ" => Ok(DeltaJ),
            "DELTAK" => Ok(DeltaK),
            "DELTAJK" => Ok(DeltaJK),
            "deltaJ" => Ok(deltaJ),
            "deltaK" => Ok(deltaK),
            _ => Err(()),
        }
    }
}

impl Display for QuartType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use QuartType::*;
        f.pad(match self {
            DeltaJ => "DELTAJ",
            DeltaK => "DELTAK",
            DeltaJK => "DELTAJK",
            deltaJ => "deltaJ",
            deltaK => "deltaK",
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Delta {
    pub(crate) typ: QuartType,
    pub(crate) val: f64,
}

impl Delta {
    pub fn new(typ: &str, val: f64) -> Self {
        Self {
            typ: typ.parse().unwrap(),
            val,
        }
    }
}
