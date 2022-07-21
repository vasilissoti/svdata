let mut a = SvPrimaryLiteralIntegral {
    data01: vec![1, 9223372036854775808],
    dataXZ: None,
    size: 65,
    signed: true,
};

a._sign_extension();

let exp = SvPrimaryLiteralIntegral {
    data01: vec![18446744073709551615, 9223372036854775808],
    dataXZ: None,
    size: 128,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);