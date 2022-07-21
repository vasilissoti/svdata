let mut a = SvPrimaryLiteralIntegral {
    data01: vec![9223372036854775808, 9223372036854775808],
    size: 128,
    signed: true,
};

a._truncate(64);

let exp = SvPrimaryLiteralIntegral {
    data01: vec![9223372036854775808],
    size: 64,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);