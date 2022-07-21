let mut a = SvPrimaryLiteral {
    data01: vec![9223372036854775809, 9223372036854775808],
    size: 128,
    signed: false,
};

a._truncate(69);

let exp = SvPrimaryLiteral {
    data01: vec![1, 9223372036854775808],
    size: 69,
    signed: false,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);