let a = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904, 4611686018427387904],
    data_xz: Some(vec![0, 0]),
    size: 128,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a + 4611686018427387904;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 4611686018427387904, 0],
    data_xz: Some(vec![0, 0, 0]),
    size: 129,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);