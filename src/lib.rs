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
        // these are for multi-line state descriptions
        let mut vib_states = Vec::new();
        let mut cur_zpt = 0.0;
        let mut cur_freq = 0.0;
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
                vib_states = Vec::new();
                let fields: Vec<_> = line.split_whitespace().collect();
                vib_states.extend(
                    fields[6..].iter().map(|s| s.parse::<usize>().unwrap()),
                );
                cur_zpt = fields[1].parse().unwrap();
                cur_freq = fields[2].parse().unwrap();
            } else if state == State::Corr && line.contains("DEGEN   (Vt)") {
                let fields: Vec<_> = line.split_whitespace().collect();
                vib_states.extend(
                    fields[3..].iter().map(|s| s.parse::<usize>().unwrap()),
                );
            } else if state == State::Corr && line.contains("DEGEN   (Vl)") {
                // nothing for now, just eat the line after handling the count
                // above in the Vt case
            } else if state == State::Corr && line.contains("<>") {
                state = State::None;
            } else if state == State::Corr && !line.is_empty() {
                let fields: Vec<_> = line.split_whitespace().collect();
                vib_states
                    .extend(fields.iter().map(|s| s.parse::<usize>().unwrap()));
            } else if state == State::Corr
                && line.is_empty()
                && vib_states.len() > 0
            {
                if vib_states.iter().all(|s| *s == 0) {
                    ret.zpt = cur_zpt;
                } else {
                    let mut one = false;
                    let mut idx = 0;
                    for (i, state) in vib_states.iter().enumerate() {
                        if (*state == 1 && one) || *state == 2 {
                            continue 'outer;
                        } else if *state == 1 {
                            idx = i;
                            one = true;
                        }
                    }
                    if idx >= ret.corr.len() {
                        ret.corr.resize(idx + 1, 0.0);
                    }
                    ret.corr[idx] = cur_freq;
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
    fn c3h2() {
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

    #[test]
    fn c2h4() {
        let got = Summary::new("testfiles/c2h4.out");
        let want = Summary {
            harm: vec![
                3247.609, 3221.841, 3154.890, 3140.072, 1670.825, 1477.408,
                1368.483, 1248.308, 1050.245, 963.438, 949.377, 825.523,
            ],
            fund: vec![
                3100.190, 3077.237, 3018.494, 3000.770, 1628.282, 1439.513,
                1341.751, 1226.454, 1024.367, 948.677, 939.365, 823.880,
            ],
            corr: vec![
                3100.1904, 3077.2369, 3015.7671, 2978.2409, 1623.0185,
                1439.5135, 1341.7506, 1226.4540, 1024.3674, 948.6771, 939.3649,
                823.8796,
            ],
            zpt: 11022.5891,
        };
        assert_eq!(got.harm.len(), want.harm.len());
        assert_eq!(got.fund.len(), want.fund.len());
        assert_eq!(got.corr.len(), want.corr.len());
        assert_eq!(got, want);
    }

    #[test]
    fn degmode() {
        let got = Summary::new("testfiles/degmode.out");
        let want = Summary {
            harm: vec![2929.500, 2834.256, 2236.673, 939.167, 791.065],
            fund: vec![2886.379, 2799.917, 2221.068, 936.105, 797.174],
            corr: vec![2886.3792, 2799.9172, 2221.0683, 936.1049, 797.1743],
            zpt: 5707.3228,
        };
        assert_eq!(got.harm, want.harm);
        assert_eq!(got.fund, want.fund);
        assert_eq!(got.corr, want.corr);
        assert_eq!(got.zpt, want.zpt);
    }
}
