let a = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904, 4611686018427387904],
    data_xz: Some(vec![0, 0]),
    size: 128,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a + 4611686018427387904;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904, 9223372036854775808],
    data_xz: Some(vec![0, 0]),
    size: 128,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);