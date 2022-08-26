let mut a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 9223372036854775809],
    data_xz: Some(vec![0, 9223372036854775809]),
    size: 128,
    signed: false,
};

a._truncate(69);

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 1],
    data_xz: Some(vec![0, 1]),
    size: 69,
    signed: false,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);