let mut a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 1],
    data_xz: Some(vec![0, 0]),
    size: 128,
    signed: true,
};

a._minimum_width();

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 1],
    data_xz: Some(vec![0, 0]),
    size: 66,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);