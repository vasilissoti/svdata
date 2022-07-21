let mut a = SvPrimaryLiteral {
    data01: vec![9223372036854775808, 9223372036854775808],
    size: 128,
    signed: true,
};

a._sign_extension();

let exp = SvPrimaryLiteral {
    data01: vec![9223372036854775808, 9223372036854775808],
    size: 128,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);