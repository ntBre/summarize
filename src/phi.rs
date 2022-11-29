use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Phi {
    // a reduction
    pub big_phi_j: Option<f64>,
    pub big_phi_k: Option<f64>,
    pub big_phi_jk: Option<f64>,
    pub big_phi_kj: Option<f64>,
    pub phi_j: Option<f64>,
    pub phi_jk: Option<f64>,
    pub phi_k: Option<f64>,
    // s reduction
    pub h_j: Option<f64>,
    pub h_jk: Option<f64>,
    pub h_kj: Option<f64>,
    pub h_k: Option<f64>,
    pub h1: Option<f64>,
    pub h2: Option<f64>,
    pub h3: Option<f64>,
    // linear molecules
    pub he: Option<f64>,
}

impl Display for Phi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write_dist_consts! {
            f, self,
            big_phi_j => "PHI J",
            big_phi_k => "PHI K",
            big_phi_jk => "PHI JK",
            big_phi_kj => "PHI KJ",
            phi_j => "phi j",
            phi_jk => "phi jk",
            phi_k => "phi k",
        }
        Ok(())
    }
}
