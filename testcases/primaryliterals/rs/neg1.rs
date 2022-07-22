let a = SvPrimaryLiteral {
    data01: vec![9223372036854775808, 9223372036854775808],
    num_bits: 128,
    signed: true,
};

let b: SvPrimaryLiteral = a.neg();

let exp = SvPrimaryLiteral {
    data01: vec![9223372036854775807, 9223372036854775808],
    num_bits: 128,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);