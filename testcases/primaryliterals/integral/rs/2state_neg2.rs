let a = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904],
    data_xz: None,
    size: 64,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = -a;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904],
    data_xz: None,
    size: 63,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);