use super::*;

use symm::Irrep::*;

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
    assert_eq!(got.harm.len(), want.harm.len());
    assert_eq!(got.fund.len(), want.fund.len());
    assert_eq!(got.corr.len(), want.corr.len());
    assert_eq!(got.lxm.len(), want.lxm.len());
    assert_eq!(got.irreps, want.irreps);
    assert_eq!(got.rots.len(), want.rots.len());
    assert_eq!(got.rots, want.rots);
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
    assert_eq!(got.harm.len(), want.harm.len());
    assert_eq!(got.fund.len(), want.fund.len());
    assert_eq!(got.corr.len(), want.corr.len());
    assert_eq!(got.lxm.len(), want.lxm.len());
    assert_eq!(got.irreps, want.irreps);
    assert_eq!(got.rots.len(), want.rots.len());
    assert_eq!(got.rots, want.rots);
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
        irreps: vec![Ag, B1u, Ag, B2u, B3g],
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
        rots: vec![],
    };
    assert_eq!(got.harm, want.harm);
    assert_eq!(got.fund, want.fund);
    assert_eq!(got.corr, want.corr);
    assert_eq!(got.zpt, want.zpt);
    assert_eq!(got.lxm.len(), want.lxm.len());
    assert_eq!(got.irreps, want.irreps);
    assert_eq!(got, want);
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
        lxm: vec![
            vec![
                0.0000000, 0.2295266, 0.0000000, 0.0000000, -0.0716598,
                0.0000000, -0.1320943, -0.1857126, 0.0000000, 0.1320943,
                -0.1857126, 0.0000000, 0.0064943, 0.4315738, 0.0000000,
                0.4304381, 0.2181238, 0.0000000, -0.0064943, 0.4315738,
                0.0000000, -0.4304381, 0.2181238, 0.0000000,
            ],
            vec![
                0.0058876, 0.0000000, 0.0000000, -0.0040133, 0.0000000,
                0.0000000, 0.1341215, 0.1930710, 0.0000000, 0.1341215,
                -0.1930710, 0.0000000, -0.0098982, -0.4379078, 0.0000000,
                -0.4489208, -0.2265359, 0.0000000, -0.0098982, 0.4379078,
                0.0000000, -0.4489208, 0.2265359, 0.0000000,
            ],
            vec![
                0.0000000, -0.9266219, 0.0000000, 0.0000000, 0.2873441,
                0.0000000, -0.0186378, -0.0582711, 0.0000000, 0.0186378,
                -0.0582711, 0.0000000, 0.0051505, 0.1155511, 0.0000000,
                0.0972846, 0.0530744, 0.0000000, -0.0051505, 0.1155511,
                0.0000000, -0.0972846, 0.0530744, 0.0000000,
            ],
            vec![
                -0.0007217, 0.0000000, 0.0000000, -0.0336568, 0.0000000,
                0.0000000, 0.1293220, -0.0994085, 0.0000000, 0.1293220,
                0.0994085, 0.0000000, 0.0143535, 0.5165148, 0.0000000,
                -0.4021319, -0.2100559, 0.0000000, 0.0143535, -0.5165148,
                0.0000000, -0.4021319, 0.2100559, 0.0000000,
            ],
            vec![
                0.0000000, 0.0258262, 0.0000000, 0.0000000, 0.0080310,
                0.0000000, 0.1300854, -0.0939612, 0.0000000, -0.1300854,
                -0.0939612, 0.0000000, 0.0121943, 0.5099021, 0.0000000,
                -0.4106093, -0.2124488, 0.0000000, -0.0121943, 0.5099021,
                0.0000000, 0.4106093, -0.2124488, 0.0000000,
            ],
            vec![
                0.2506678, 0.0000000, 0.0000000, -0.6480491, 0.0000000,
                0.0000000, 0.3605970, -0.1544138, 0.0000000, 0.3605970,
                0.1544138, 0.0000000, -0.2601215, -0.0517110, 0.0000000,
                0.0085644, 0.1852420, 0.0000000, -0.2601215, 0.0517110,
                0.0000000, 0.0085644, -0.1852420, 0.0000000,
            ],
            vec![
                0.0000000, -0.0757636, 0.0000000, 0.0000000, -0.2060881,
                0.0000000, -0.2818950, 0.2172804, 0.0000000, 0.2818950,
                0.2172804, 0.0000000, 0.3861922, 0.0588077, 0.0000000,
                0.1558539, -0.4151204, 0.0000000, -0.3861922, 0.0588077,
                0.0000000, -0.1558539, -0.4151204, 0.0000000,
            ],
            vec![
                0.3418837, 0.0000000, 0.0000000, -0.4269140, 0.0000000,
                0.0000000, -0.0048458, 0.0912667, 0.0000000, -0.0048458,
                -0.0912667, 0.0000000, 0.3519927, 0.0092858, 0.0000000,
                0.2301814, -0.4063353, 0.0000000, 0.3519927, -0.0092858,
                0.0000000, 0.2301814, 0.4063353, 0.0000000,
            ],
            vec![
                0.0000000, -0.2016936, 0.0000000, 0.0000000, -0.6124450,
                0.0000000, -0.1096599, 0.2997010, 0.0000000, 0.1096599,
                0.2997010, 0.0000000, -0.4230917, 0.1039268, 0.0000000,
                -0.0092110, 0.0194220, 0.0000000, 0.4230917, 0.1039268,
                0.0000000, 0.0092110, 0.0194220, 0.0000000,
            ],
            vec![
                -0.7925676, 0.0000000, 0.0000000, -0.2271384, 0.0000000,
                0.0000000, 0.1469657, -0.2574649, 0.0000000, 0.1469657,
                0.2574649, 0.0000000, 0.2466423, -0.0916527, 0.0000000,
                0.0344609, -0.0426036, 0.0000000, 0.2466423, 0.0916527,
                0.0000000, 0.0344609, 0.0426036, 0.0000000,
            ],
            vec![
                0.0000000, 0.0000000, 0.0000000, 0.0000000, 0.0000000,
                0.0000000, 0.0000000, 0.0000000, 0.3621097, 0.0000000,
                0.0000000, -0.3621097, 0.0000000, 0.0000000, -0.3454082,
                0.0000000, 0.0000000, -0.4995695, 0.0000000, 0.0000000,
                0.3454082, 0.0000000, 0.0000000, 0.4995695,
            ],
            vec![
                0.0000000, 0.0000000, 0.0037754, 0.0000000, 0.0000000,
                -0.2449837, 0.0000000, 0.0000000, 0.3601275, 0.0000000,
                0.0000000, 0.3601275, 0.0000000, 0.0000000, -0.4463719,
                0.0000000, 0.0000000, -0.3755591, 0.0000000, 0.0000000,
                -0.4463719, 0.0000000, 0.0000000, -0.3755591,
            ],
            vec![
                0.0000000, -0.0309968, 0.0000000, 0.0000000, -0.0816821,
                0.0000000, -0.4766293, -0.0527489, 0.0000000, 0.4766293,
                -0.0527489, 0.0000000, 0.1727615, -0.0137427, 0.0000000,
                -0.3348719, 0.3522089, 0.0000000, -0.1727615, -0.0137427,
                0.0000000, 0.3348719, 0.3522089, 0.0000000,
            ],
            vec![
                0.0000000, 0.0000000, 0.8526852, 0.0000000, 0.0000000,
                -0.3720691, 0.0000000, 0.0000000, 0.0301895, 0.0000000,
                0.0000000, 0.0301895, 0.0000000, 0.0000000, 0.2290900,
                0.0000000, 0.0000000, -0.1176972, 0.0000000, 0.0000000,
                0.2290900, 0.0000000, 0.0000000, -0.1176972,
            ],
            vec![
                -0.3500804, 0.0000000, 0.0000000, 0.0141098, 0.0000000,
                0.0000000, 0.1015384, 0.2331608, 0.0000000, 0.1015384,
                -0.2331608, 0.0000000, -0.4377546, 0.0763989, 0.0000000,
                0.2380526, -0.3461407, 0.0000000, -0.4377546, -0.0763989,
                0.0000000, 0.2380526, 0.3461407, 0.0000000,
            ],
            vec![
                0.0000000, 0.0000000, 0.0000000, 0.0000000, 0.0000000,
                0.0000000, 0.0000000, 0.0000000, -0.0339031, 0.0000000,
                0.0000000, 0.0339031, 0.0000000, 0.0000000, -0.5922102,
                0.0000000, 0.0000000, 0.3848866, 0.0000000, 0.0000000,
                0.5922102, 0.0000000, 0.0000000, -0.3848866,
            ],
            vec![
                0.0000000, 0.1268194, 0.0000000, 0.0000000, 0.4422595,
                0.0000000, -0.3829046, -0.1451511, 0.0000000, 0.3829046,
                -0.1451511, 0.0000000, -0.3765436, -0.0372292, 0.0000000,
                0.0120122, -0.2883775, 0.0000000, 0.3765436, -0.0372292,
                0.0000000, -0.0120122, -0.2883775, 0.0000000,
            ],
            vec![
                0.0000000, 0.0000000, 0.0651349, 0.0000000, 0.0000000,
                -0.4724962, 0.0000000, 0.0000000, 0.1322916, 0.0000000,
                0.0000000, 0.1322916, 0.0000000, 0.0000000, -0.2345848,
                0.0000000, 0.0000000, 0.5601083, 0.0000000, 0.0000000,
                -0.2345848, 0.0000000, 0.0000000, 0.5601083,
            ],
        ],
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
    assert_eq!(got.harm, want.harm);
    assert_eq!(got.fund, want.fund);
    assert_eq!(got.corr, want.corr);
    assert_eq!(got.zpt, want.zpt);
    assert_eq!(got.lxm.len(), want.lxm.len());
    assert_eq!(got.lxm, want.lxm);
    assert_eq!(got.irreps, want.irreps);
    assert_eq!(got.rots.len(), want.rots.len());
    assert_eq!(got.rots, want.rots);
    assert_eq!(got, want);
}
