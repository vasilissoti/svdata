let a = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904],
    data_xz: None,
    size: 64,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a.lsr(4);

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![288230376151711744],
    data_xz: None,
    size: 64,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);