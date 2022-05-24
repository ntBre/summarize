use std::io::{BufRead, BufReader};

#[derive(Default, Debug, PartialEq)]
pub struct Summary {
    // pub lx: Vec<f64>,
    pub harm: Vec<f64>,
    pub fund: Vec<f64>,
    pub corr: Vec<f64>,
    // pub rots: Vec<Vec<f64>>,
    // pub deltas: Vec<f64>,
    // pub phis: Vec<f64>,
    // pub rhead: Vec<String>,
    // pub ralpha: Vec<f64>,
    // pub requil: Vec<f64>,
    // pub fermi: Vec<String>,
    pub zpt: f64,
    // pub lin: bool,
    // pub imag: bool,
}

impl Summary {
    pub fn new(filename: &str) -> Self {
        let f = match std::fs::File::open(filename) {
            Ok(f) => f,
            Err(e) => panic!("failed to open {} with '{}'", filename, e),
        };
        let lines = BufReader::new(f).lines().flatten();
        #[derive(PartialEq)]
        enum State {
            Fund,
            Corr,
            None,
        }
        let mut state = State::None;
        let mut skip = 0;
        let mut ret = Self::default();
        'outer: for line in lines {
            if skip > 0 {
                skip -= 1;
            } else if line.contains("BAND CENTER ANALYSIS") {
                skip = 3;
                state = State::Fund;
            } else if line.contains("DUNHAM")
                || line.contains("VIBRATIONAL ENERGY AND")
            {
                state = State::None;
            } else if state == State::Fund
                && line.contains(|s: char| s.is_numeric())
            {
                let fields: Vec<_> = line.split_whitespace().collect();
                ret.harm.push(fields[1].parse().unwrap());
                ret.fund.push(fields[2].parse().unwrap());
            } else if line.contains("STATE NO.") && !line.contains("SPECTRUM") {
                skip = 2;
                state = State::Corr;
            } else if state == State::Corr
                && line.contains("*******************")
            {
                state = State::None;
            } else if state == State::Corr && line.contains("NON-DEG (Vs)") {
                // TODO handle multi-line state descriptions, simple for now
                let fields: Vec<_> = line.split_whitespace().collect();
                {
                    let mut one = false;
                    for f in &fields[6..] {
                        if *f == "2" || (one && *f == "1") {
                            continue 'outer;
                        } else if *f == "1" {
                            one = true;
                        } // append to state vector here when I add that
                    }
                }
                if fields[6..].iter().all(|s| *s == "0") {
                    ret.zpt = fields[1].parse().unwrap();
                } else {
                    let idx =
                        fields[6..].iter().position(|&x| x == "1").unwrap();
                    if idx >= ret.corr.len() {
                        ret.corr.resize(idx + 1, 0.0);
                    }
                    ret.corr[idx] = fields[2].parse().unwrap();
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c3h2() {
        let got = Summary::new("testfiles/spectro.out");
        let want = Summary {
            harm: vec![
                3281.362, 3247.646, 1623.590, 1307.445, 1090.564, 992.798,
                908.650, 901.695, 785.141,
            ],
            fund: vec![
                3152.935, 3108.684, 1593.567, 1275.793, 1056.887, 1007.899,
                876.800, 876.478, 772.658,
            ],
            corr: vec![
                3139.8162, 3108.6836, 1595.1229, 1275.7931, 1056.8867,
                1007.8986, 876.8004, 876.4785, 772.6584,
            ],
            zpt: 6993.7720,
        };
        assert_eq!(got, want);
    }
}
