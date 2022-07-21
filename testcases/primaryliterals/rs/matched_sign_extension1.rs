let mut a = SvPrimaryLiteral {
    data01: vec![9223372036854775808, 9223372036854775808],
    size: 128,
    signed: true,
};

let mut b = SvPrimaryLiteral {
    data01: vec![0, 9223372036854775808],
    size: 65,
    signed: true,
};

a._matched_sign_extension(&mut b);

let exp = SvPrimaryLiteral {
    data01: vec![9223372036854775808, 9223372036854775808],
    size: 128,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);