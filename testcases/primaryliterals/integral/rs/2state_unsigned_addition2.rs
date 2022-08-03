let a = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904],
    data_xz: None,
    size: 63,
    signed: false,
};

let b: SvPrimaryLiteralIntegral = a + 4611686018427387904;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808],
    data_xz: None,
    size: 64,
    signed: false,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);