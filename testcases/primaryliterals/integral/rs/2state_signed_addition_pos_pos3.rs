let a = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904, 4611686018427387904],
    data_xz: None,
    size: 128,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904],
    data_xz: None,
    size: 64,
    signed: true,
};

let c: SvPrimaryLiteralIntegral = a + b;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0, 4611686018427387904, 9223372036854775808],
    data_xz: None,
    size: 129,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);