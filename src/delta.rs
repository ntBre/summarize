use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, Default, PartialEq, Serialize)]
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
