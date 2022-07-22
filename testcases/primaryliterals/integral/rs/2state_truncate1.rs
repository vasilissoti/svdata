let mut a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 4611686018427387905],
    data_xz: None,
    size: 128,
    signed: true,
};

a._truncate(5);

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![1],
    data_xz: None,
    size: 5,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);