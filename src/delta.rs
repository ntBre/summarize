use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Delta {
    /// these constants are in the A-reduced Watson Hamiltonian
    pub big_delta_j: Option<f64>,
    pub big_delta_k: Option<f64>,
    pub big_delta_jk: Option<f64>,
    pub delta_j: Option<f64>,
    pub delta_k: Option<f64>,

    /// these constants are in the S-reduced Watson Hamiltonian
    pub d_j: Option<f64>,
    pub d_jk: Option<f64>,
    pub d_k: Option<f64>,
    pub d1: Option<f64>,
    pub d2: Option<f64>,

    /// linear molecules only
    pub de: Option<f64>,
}

impl Delta {
    /// return the unwrapped fields of `self` as a single vector
    pub fn to_vec(&self) -> Vec<f64> {
        vec![
            self.big_delta_j.unwrap_or(0.0),
            self.big_delta_k.unwrap_or(0.0),
            self.big_delta_jk.unwrap_or(0.0),
            self.delta_j.unwrap_or(0.0),
            self.delta_k.unwrap_or(0.0),
            self.d_j.unwrap_or(0.0),
            self.d_jk.unwrap_or(0.0),
            self.d_k.unwrap_or(0.0),
            self.d1.unwrap_or(0.0),
            self.d2.unwrap_or(0.0),
            self.de.unwrap_or(0.0),
        ]
    }
}

impl Display for Delta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write_dist_consts!(f, self,
        big_delta_j => "DELTA J",
        big_delta_k => "DELTA K",
        big_delta_jk => "DELTA JK",
        delta_j => "delta J",
        delta_k => "delta K",
           );

        Ok(())
    }
}
