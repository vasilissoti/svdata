let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 0],
    data_xz: Some(vec![9223372036854775808, 9223372036854775808]),
    size: 128,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a << 1;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![1, 0, 0],
    data_xz: Some(vec![1, 1, 0]),
    size: 129,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);