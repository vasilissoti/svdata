let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 9223372036854775808],
    data_xz: None,
    size: 128,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808],
    data_xz: None,
    size: 64,
    signed: true,
};

let c: SvPrimaryLiteralIntegral = a + b;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0, 9223372036854775808, 1],
    data_xz: None,
    size: 129,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);