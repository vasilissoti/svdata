let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808],
    data_xz: Some(vec![0]),
    size: 64,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a + 9223372036854775808;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0, 1],
    data_xz: Some(vec![0, 0]),
    size: 65,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);