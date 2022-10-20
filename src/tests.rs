use std::fs::read_to_string;

use super::*;

use symm::Irrep::*;

/// load a "matrix" (Vec<Vec<f64>>) from `filename`
fn load_mat(filename: &str) -> Vec<Vec<f64>> {
    let data = read_to_string(filename).unwrap();
    let mut ret = Vec::new();
    for line in data.lines() {
        let v: Vec<_> = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<f64>().unwrap())
            .collect();
        if !v.is_empty() {
            ret.push(v);
        }
    }
    ret
}

macro_rules! check {
    ($got:expr, $want:expr) => {
        assert_eq!($got.harm.len(), $want.harm.len());
        assert_eq!($got.fund.len(), $want.fund.len());
        assert_eq!($got.corr.len(), $want.corr.len());
        assert_eq!($got.lxm.len(), $want.lxm.len());
        assert_eq!($got.irreps, $want.irreps);
        assert_eq!($got.rots.len(), $want.rots.len());
        assert_eq!($got.rots, $want.rots);
        assert_eq!($got, $want);
    };
}

#[test]
fn c3h2() {
    let got = Summary::new("testfiles/spectro.out");
    let want = Summary {
        harm: vec![
            3281.362, 3247.646, 1623.590, 1307.445, 1090.564, 992.798, 908.650,
            901.695, 785.141,
        ],
        fund: vec![
            3152.935, 3108.684, 1593.567, 1275.793, 1056.887, 1007.899,
            876.800, 876.478, 772.658,
        ],
        corr: vec![
            3139.8162, 3108.6836, 1595.1229, 1275.7931, 1056.8867, 1007.8986,
            876.8004, 876.4785, 772.6584,
        ],
        zpt: 6993.7720,
        irreps: vec![A1, B2, A1, A1, B2, A2, B2, A1, B1],
        geom: Molecule::from_str(
            "
C                  0.0000000     -0.8888444      0.0000000
C                 -0.6626966      0.3682537      0.0000000
C                  0.6626966      0.3682537      0.0000000
H                 -1.5951933      0.9069249      0.0000000
H                  1.5951933      0.9069249      0.0000000
",
        )
        .unwrap(),
        lxm: load_mat("testfiles/c3h2.lxm"),
        rots: vec![
            vec![1.0699983, 0.5558117, 1.1639391],
            vec![1.0655950, 0.5541780, 1.1620074],
            vec![1.0658789, 0.5542868, 1.1620312],
            vec![1.0638367, 0.5543723, 1.1642112],
            vec![1.0703606, 0.5561731, 1.1608226],
            vec![1.0712611, 0.5513020, 1.1594914],
            vec![1.0662658, 0.5563234, 1.1647092],
            vec![1.0729460, 0.5538695, 1.1622924],
            vec![1.0723935, 0.5553067, 1.1657756],
            vec![1.0697371, 0.5566012, 1.1595315],
        ],
    };
    check!(got, want);
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
            3100.1904, 3077.2369, 3015.7671, 2978.2409, 1623.0185, 1439.5135,
            1341.7506, 1226.4540, 1024.3674, 948.6771, 939.3649, 823.8796,
        ],
        zpt: 11022.5891,
        irreps: vec![B2u, B3g, Ag, B1u, Ag, B1u, Ag, B3g, Au, B3u, B2g, B2u],
        geom: Molecule::from_str(
            "
    C              0.6667933      0.0000000      0.0000000
    H              1.2309854     -0.9236110      0.0000000
    H              1.2309854      0.9236110      0.0000000
    C             -0.6667933      0.0000000      0.0000000
    H             -1.2309854     -0.9236110      0.0000000
    H             -1.2309854      0.9236110      0.0000000
",
        )
        .unwrap(),
        lxm: load_mat("testfiles/c2h4.lxm"),
        rots: vec![
            vec![0.9970438, 0.8248191, 4.8497526],
            vec![0.9947709, 0.8228383, 4.8246085],
            vec![0.9949339, 0.8231722, 4.8233596],
            vec![0.9955674, 0.8226294, 4.8077994],
            vec![0.9958420, 0.8228567, 4.8148440],
            vec![0.9942198, 0.8200136, 4.8631492],
            vec![1.0034126, 0.8232445, 4.9141218],
            vec![0.9978865, 0.8227350, 4.8613105],
            vec![0.9985819, 0.8226920, 4.9864849],
            vec![1.0089357, 0.8248270, 4.7600134],
            vec![0.9915086, 0.8261680, 4.8147661],
            vec![0.9914912, 0.8255070, 4.7225486],
            vec![0.9821289, 0.8232440, 4.8995346],
        ],
    };
    check!(got, want);
}

#[test]
fn degmode() {
    let got = Summary::new("testfiles/degmode.out");
    let want = Summary {
        harm: vec![2929.500, 2834.256, 2236.673, 939.167, 791.065],
        fund: vec![2886.379, 2799.917, 2221.068, 936.105, 797.174],
        corr: vec![2886.3792, 2799.9172, 2221.0683, 936.1049, 797.1743],
        zpt: 5707.3228,
        irreps: vec![Ag, B1u, Ag, B2u, B3u, B3g, B2g],
        geom: Molecule::from_str(
            "
                 H   0.0000000      0.0000000      1.6353253
                 C   0.0000000      0.0000000     -0.6014244
                 C   0.0000000      0.0000000      0.6014244
                 H   0.0000000      0.0000000     -1.6353253
",
        )
        .unwrap(),
        lxm: load_mat("testfiles/degmode.lxm"),
        rots: vec![],
    };
    check!(got, want);
}

#[test]
fn allyl() {
    let got = Summary::new("testfiles/allyl.out");
    let want = Summary {
        harm: vec![
            3253.47, 3250.966, 3214.569, 3141.939, 3139.941, 1625.315, 1556.29,
            1451.065, 1320.264, 1296.008, 1153.123, 1145.001, 1056.978,
            1041.647, 953.372, 641.32, 443.764, 290.685,
        ],
        fund: vec![
            3109.857, 3105.617, 3072.258, 3007.378, 3003.555, 1587.556,
            1517.298, 1413.193, 1292.210, 1271.072, 1128.549, 1119.805,
            1035.561, 1008.711, 940.006, 622.837, 432.824, 188.325,
        ],
        corr: vec![
            3109.8572, 3105.6171, 3072.2577, 3011.8158, 2991.8791, 1585.8923,
            1518.4918, 1413.1933, 1292.4703, 1269.5049, 1136.4317, 1119.8048,
            1035.8188, 1008.7108, 940.0058, 622.8365, 439.1814, 188.3254,
        ],
        geom: Molecule::from_str(
            "
H    0.0000000     -1.5747877      0.0000000
C    0.0000000     -0.4928627      0.0000000
C   -1.1790111      0.2248631      0.0000000
C    1.1790111      0.2248631      0.0000000
H   -1.1614419      1.3102804      0.0000000
H   -2.1461889     -0.2660770      0.0000000
H    1.1614419      1.3102804      0.0000000
H    2.1461889     -0.2660770      0.0000000

",
        )
        .unwrap(),
        irreps: vec![
            A1, B2, A1, B2, A1, B2, A1, B2, A1, B2, A2, B1, A1, B1, B2, A2, A1,
            B1,
        ],
        lxm: load_mat("testfiles/allyl.lxm"),
        rots: vec![
            vec![0.3689075, 0.3007544, 1.6334856],
            vec![0.3684537, 0.3003894, 1.6303351],
            vec![0.3685153, 0.3004401, 1.6301555],
            vec![0.3683393, 0.3002424, 1.6285789],
            vec![0.3685665, 0.3003826, 1.6293036],
            vec![0.3685267, 0.3003600, 1.6296515],
            vec![0.3686596, 0.3000130, 1.6263064],
            vec![0.3687563, 0.3000184, 1.6436374],
            vec![0.3695578, 0.3003670, 1.6416106],
            vec![0.3679214, 0.2996553, 1.6412361],
            vec![0.3702732, 0.3007958, 1.6267719],
            vec![0.3680588, 0.3007986, 1.6214341],
            vec![0.3679041, 0.3007661, 1.6246146],
            vec![0.3684392, 0.3001902, 1.6433634],
            vec![0.3680857, 0.3009425, 1.6202227],
            vec![0.3687611, 0.3002564, 1.6419496],
            vec![0.3663138, 0.3000918, 1.6363695],
            vec![0.3678643, 0.2998220, 1.6520336],
            vec![0.3719477, 0.3031271, 1.6067642],
        ],
        zpt: 14773.0842,
    };
    check!(got, want);
}

#[test]
fn c3h3() {
    let got = Summary::new("testfiles/c3h3.out");
    let want = Summary {
        harm: vec![
            2703.543, 2654.315, 2654.289, 1840.347, 1187.899, 1187.834,
            1013.010, 967.949, 967.947, 928.691, 928.613, 907.075,
        ],
        fund: vec![
            2663.113, 2613.774, 2613.746, 1805.303, 1163.935, 1163.874,
            1001.584, 962.582, 962.578, 921.084, 921.025, 902.798,
        ],
        corr: vec![
            2663.1126, 2613.7738, 2613.7457, 1787.6737, 1163.9351, 1163.8742,
            1001.5837, 962.582, 962.5782, 921.0844, 921.0251, 902.7976,
        ],
        geom: Molecule::from_str(
            "
C   -0.5752253      0.5636900      0.0000000
C   -0.2005581     -0.7800038      0.0000000
C    0.7757834      0.2163138      0.0000000
H    1.8160258      0.5063683      0.0000000
H   -0.4694853     -1.8259092      0.0000000
H   -1.3465409      1.3195405      0.0000000
",
        )
        .unwrap(),
        irreps: vec![Ap, Ap, Ap, Ap, Ap, Ap, Ap, App, App, Ap, Ap, App],
        lxm: load_mat("testfiles/c3h3.lxm"),
        rots: vec![
            vec![0.9870171, 0.4922263, 0.9870160],
            vec![0.9863721, 0.4919151, 0.9863710],
            vec![0.9865655, 0.4925811, 0.9866069],
            vec![0.9866080, 0.4914622, 0.9865644],
            vec![0.9845928, 0.4910162, 0.9845917],
            vec![0.9886483, 0.4910968, 0.9883862],
            vec![0.9883887, 0.4910965, 0.9886488],
            vec![0.9877622, 0.4917814, 0.9877609],
            vec![0.9847201, 0.4927154, 0.9838032],
            vec![0.9838042, 0.4927154, 0.9847177],
            vec![0.9887181, 0.6220108, 1.0254229],
            vec![0.9887175, 0.3612145, 1.0334698],
            vec![0.9857452, 0.4927646, 0.9042860],
        ],
        zpt: 8904.3886,
    };
    check!(got, want);
}
