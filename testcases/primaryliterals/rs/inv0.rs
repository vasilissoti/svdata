let a = SvPrimaryLiteral {
    data01: vec![0, 9223372036854775808],
    num_bits: 65,
    signed: true,
};

let b: SvPrimaryLiteral = a.inv();

let exp = SvPrimaryLiteral {
    data01: vec![1, 9223372036854775807],
    num_bits: 65,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);