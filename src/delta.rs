use std::fmt::Display;

#[derive(Debug, Default, PartialEq)]
pub struct Delta {
    pub big_delta_j: Option<f64>,
    pub big_delta_k: Option<f64>,
    pub big_delta_jk: Option<f64>,
    pub delta_j: Option<f64>,
    pub delta_k: Option<f64>,
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
