use super::*;

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
        irreps: vec![],
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
        lxm: vec![
            vec![
                0.0000000, 0.0252541, 0.0000000, 0.2238580, -0.1089589,
                0.0000000, -0.2238580, -0.1089589, 0.0000000, -0.5720169,
                0.3324053, 0.0000000, 0.5720169, 0.3324053, 0.0000000,
            ],
            vec![
                -0.0062392, 0.0000000, 0.0000000, 0.1738758, -0.1009703,
                0.0000000, 0.1738758, 0.1009703, 0.0000000, -0.5892159,
                0.3352273, 0.0000000, -0.5892159, -0.3352273, 0.0000000,
            ],
            vec![
                0.0000000, 0.2542934, 0.0000000, 0.6357618, -0.0906171,
                0.0000000, -0.6357618, -0.0906171, 0.0000000, 0.1984296,
                -0.1260501, 0.0000000, -0.1984296, -0.1260501, 0.0000000,
            ],
            vec![
                0.0000000, -0.7162329, 0.0000000, 0.2025466, 0.4052844,
                0.0000000, -0.2025466, 0.4052844, 0.0000000, -0.1083246,
                -0.1627588, 0.0000000, 0.1083246, -0.1627588, 0.0000000,
            ],
            vec![
                -0.4012448, 0.0000000, 0.0000000, 0.1316601, -0.5604063,
                0.0000000, 0.1316601, 0.5604063, 0.0000000, 0.2379631,
                0.1774409, 0.0000000, 0.2379631, -0.1774409, 0.0000000,
            ],
            vec![
                0.0000000, 0.0000000, 0.0000000, 0.0000000, 0.0000000,
                0.4045607, 0.0000000, 0.0000000, -0.4045607, 0.0000000,
                0.0000000, -0.5799402, 0.0000000, 0.0000000, 0.5799402,
            ],
            vec![
                -0.4560047, 0.0000000, 0.0000000, 0.2874739, -0.0211315,
                0.0000000, 0.2874739, 0.0211315, 0.0000000, -0.2052143,
                -0.5204125, 0.0000000, -0.2052143, 0.5204125, 0.0000000,
            ],
            vec![
                0.0000000, -0.3256501, 0.0000000, 0.0683335, -0.0012698,
                0.0000000, -0.0683335, -0.0012698, 0.0000000, 0.3488382,
                0.5662304, 0.0000000, -0.3488382, 0.5662304, 0.0000000,
            ],
            vec![
                0.0000000, 0.0000000, 0.1601691, 0.0000000, 0.0000000,
                -0.2669780, 0.0000000, 0.0000000, -0.2669780, 0.0000000,
                0.0000000, 0.6448997, 0.0000000, 0.0000000, 0.6448997,
            ],
        ],
    };
    assert_eq!(got.harm.len(), want.harm.len());
    assert_eq!(got.fund.len(), want.fund.len());
    assert_eq!(got.corr.len(), want.corr.len());
    assert_eq!(got.lxm.len(), want.lxm.len());
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
            3100.1904, 3077.2369, 3015.7671, 2978.2409, 1623.0185, 1439.5135,
            1341.7506, 1226.4540, 1024.3674, 948.6771, 939.3649, 823.8796,
        ],
        zpt: 11022.5891,
        irreps: vec![],
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
        lxm: vec![
            vec![
                0.0000000, 0.2323727, 0.0000000, 0.2495352, -0.4009158,
                0.0000000, -0.2495352, -0.4009158, 0.0000000, 0.0000000,
                0.2323729, 0.0000000, -0.2495353, -0.4009161, 0.0000000,
                0.2495353, -0.4009161, 0.0000000,
            ],
            vec![
                0.0000000, -0.2292318, 0.0000000, -0.2497991, 0.4016553,
                0.0000000, 0.2497991, 0.4016553, 0.0000000, 0.0000000,
                0.2292317, 0.0000000, -0.2497989, -0.4016550, 0.0000000,
                0.2497989, -0.4016550, 0.0000000,
            ],
            vec![
                0.1812743, 0.0000000, 0.0000000, -0.2463582, 0.4157852,
                0.0000000, -0.2463582, -0.4157852, 0.0000000, -0.1812744,
                0.0000000, 0.0000000, 0.2463583, 0.4157854, 0.0000000,
                0.2463583, -0.4157854, 0.0000000,
            ],
            vec![
                0.1430702, 0.0000000, 0.0000000, -0.2468409, 0.4228890,
                0.0000000, -0.2468409, -0.4228890, 0.0000000, 0.1430701,
                0.0000000, 0.0000000, -0.2468408, -0.4228889, 0.0000000,
                -0.2468408, 0.4228889, 0.0000000,
            ],
            vec![
                -0.5904152, 0.0000000, 0.0000000, 0.1602557, 0.2236584,
                0.0000000, 0.1602557, -0.2236584, 0.0000000, 0.5904151,
                0.0000000, 0.0000000, -0.1602557, 0.2236584, 0.0000000,
                -0.1602557, -0.2236584, 0.0000000,
            ],
            vec![
                -0.2267995, 0.0000000, 0.0000000, 0.3913005, 0.2667676,
                0.0000000, 0.3913005, -0.2667676, 0.0000000, -0.2267997,
                0.0000000, 0.0000000, 0.3913004, -0.2667676, 0.0000000,
                0.3913004, 0.2667676, 0.0000000,
            ],
            vec![
                -0.3443103, 0.0000000, 0.0000000, -0.4045067, -0.1646192,
                0.0000000, -0.4045067, 0.1646192, 0.0000000, 0.3443103,
                0.0000000, 0.0000000, 0.4045068, -0.1646193, 0.0000000,
                0.4045068, 0.1646193, 0.0000000,
            ],
            vec![
                0.0000000, -0.4284463, 0.0000000, 0.3808913, 0.1146245,
                0.0000000, -0.3808913, 0.1146245, 0.0000000, 0.0000000,
                0.4284462, 0.0000000, 0.3808912, -0.1146244, 0.0000000,
                -0.3808912, -0.1146244, 0.0000000,
            ],
            vec![
                0.0000000, 0.0000000, 0.0000000, 0.0000000, 0.0000000,
                0.5000000, 0.0000000, 0.0000000, -0.5000000, 0.0000000,
                0.0000000, 0.0000000, 0.0000000, 0.0000000, -0.5000000,
                0.0000000, 0.0000000, 0.5000000,
            ],
            vec![
                0.0000000, 0.0000000, -0.2681578, 0.0000000, 0.0000000,
                0.4626541, 0.0000000, 0.0000000, 0.4626541, 0.0000000,
                0.0000000, -0.2681522, 0.0000000, 0.0000000, 0.4626489,
                0.0000000, 0.0000000, 0.4626489,
            ],
            vec![
                0.0000000, 0.0000000, -0.4266481, 0.0000000, 0.0000000,
                0.3987261, 0.0000000, 0.0000000, 0.3987261, 0.0000000,
                0.0000000, 0.4266516, 0.0000000, 0.0000000, -0.3987322,
                0.0000000, 0.0000000, -0.3987322,
            ],
            vec![
                0.0000000, 0.1338283, 0.0000000, -0.4332807, -0.2308957,
                0.0000000, 0.4332807, -0.2308957, 0.0000000, 0.0000000,
                0.1338283, 0.0000000, 0.4332807, -0.2308957, 0.0000000,
                -0.4332807, -0.2308957, 0.0000000,
            ],
        ],
    };
    assert_eq!(got.harm.len(), want.harm.len());
    assert_eq!(got.fund.len(), want.fund.len());
    assert_eq!(got.corr.len(), want.corr.len());
    assert_eq!(got.lxm.len(), want.lxm.len());
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
        irreps: vec![],
        geom: Molecule::from_str(
            "
                 H   0.0000000      0.0000000      1.6353253
                 C   0.0000000      0.0000000     -0.6014244
                 C   0.0000000      0.0000000      0.6014244
                 H   0.0000000      0.0000000     -1.6353253
",
        )
        .unwrap(),
        lxm: vec![
            vec![
                0.0000000, 0.0000000, -0.5906415, 0.0000000, 0.0000000,
                -0.3887706, 0.0000000, 0.0000000, 0.3887706, 0.0000000,
                0.0000000, 0.5906415,
            ],
            vec![
                0.0000000, 0.0000000, -0.6791618, 0.0000000, 0.0000000,
                0.1968227, 0.0000000, 0.0000000, 0.1968227, 0.0000000,
                0.0000000, -0.6791619,
            ],
            vec![
                0.0000000, 0.0000000, 0.3887706, 0.0000000, 0.0000000,
                -0.5906415, 0.0000000, 0.0000000, 0.5906415, 0.0000000,
                0.0000000, -0.3887706,
            ],
            vec![
                -0.6791618, 0.0000000, 0.0000000, 0.1968227, 0.0000000,
                0.0000000, 0.1968227, 0.0000000, 0.0000000, -0.6791618,
                0.0000000, 0.0000000,
            ],
            vec![
                -0.5553943, 0.0000000, 0.0000000, -0.4376496, 0.0000000,
                0.0000000, 0.4376496, 0.0000000, 0.0000000, 0.5553943,
                0.0000000, 0.0000000,
            ],
        ],
    };
    assert_eq!(got.harm, want.harm);
    assert_eq!(got.fund, want.fund);
    assert_eq!(got.corr, want.corr);
    assert_eq!(got.zpt, want.zpt);
    assert_eq!(got.lxm.len(), want.lxm.len());
    assert_eq!(got, want);
}
