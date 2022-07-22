let mut a = SvPrimaryLiteral {
    data01: vec![1, 9223372036854775808],
    num_bits: 128,
    signed: true,
};

a._minimum_width();

let exp = SvPrimaryLiteral {
    data01: vec![1, 9223372036854775808],
    num_bits: 66,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);