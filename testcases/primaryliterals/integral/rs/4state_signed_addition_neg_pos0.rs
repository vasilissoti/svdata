let a = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904],
    data_xz: Some(vec![0]),
    size: 63,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a + 4611686018427387904;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0],
    data_xz: Some(vec![0]),
    size: 1,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);