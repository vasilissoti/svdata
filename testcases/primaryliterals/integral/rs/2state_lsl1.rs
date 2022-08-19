let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 9223372036854775808],
    data_xz: None,
    size: 128,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a << 2;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0, 2, 2],
    data_xz: None,
    size: 130,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);