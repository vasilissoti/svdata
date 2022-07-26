let mut a = SvPrimaryLiteralIntegral {
    data_01: vec![1, 9223372036854775808],
    data_xz: Some(vec![0, 0]),
    size: 65,
    signed: true,
};

a._sign_extend();

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![18446744073709551615, 9223372036854775808],
    data_xz: Some(vec![0, 0]),
    size: 128,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);