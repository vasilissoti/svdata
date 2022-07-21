let mut a = SvPrimaryLiteral {
    data01: vec![9223372036854775808],
    size: 64,
    signed: true,
};

a._minimum_width();

let exp = SvPrimaryLiteral {
    data01: vec![9223372036854775808],
    size: 64,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);