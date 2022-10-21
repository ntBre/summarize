use std::fmt::Display;

#[derive(Debug, Default, PartialEq)]
pub struct Phi {
    pub big_phi_j: Option<f64>,
    pub big_phi_k: Option<f64>,
    pub big_phi_jk: Option<f64>,
    pub big_phi_kj: Option<f64>,
    pub phi_j: Option<f64>,
    pub phi_jk: Option<f64>,
    pub phi_k: Option<f64>,
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
