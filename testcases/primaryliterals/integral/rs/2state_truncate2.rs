let mut a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775809, 9223372036854775808],
    data_xz: None,
    size: 128,
    signed: false,
};

a._truncate(69);

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![1, 9223372036854775808],
    data_xz: None,
    size: 69,
    signed: false,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);