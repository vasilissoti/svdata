let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808],
    data_xz: None,
    size: 64,
    signed: false,
};

let b: SvPrimaryLiteralIntegral = a + 9223372036854775808;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0, 1],
    data_xz: None,
    size: 65,
    signed: false,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);