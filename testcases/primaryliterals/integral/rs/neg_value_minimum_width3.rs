let mut a = SvPrimaryLiteralIntegral {
    data01: vec![1, 9223372036854775808],
    dataXZ: None,
    size: 128,
    signed: true,
};

a._minimum_width();

let exp = SvPrimaryLiteralIntegral {
    data01: vec![1, 9223372036854775808],
    dataXZ: None,
    size: 66,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);