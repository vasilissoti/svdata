let a = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904],
    data_xz: Some(vec![0]),
    size: 64,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a << 4;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![4, 0],
    data_xz: Some(vec![0, 0]),
    size: 68,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);