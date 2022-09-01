let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 9223372036854775808],
    data_xz: Some(vec![0, 0]),
    size: 128,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = -a;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 9223372036854775807],
    data_xz: Some(vec![0, 0]),
    size: 128,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);