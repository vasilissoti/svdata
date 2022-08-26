let mut a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 1],
    data_xz: None,
    size: 65,
    signed: true,
};

a._sign_extend();

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 18446744073709551615],
    data_xz: None,
    size: 128,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);