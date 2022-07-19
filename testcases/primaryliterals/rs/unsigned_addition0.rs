let a = SvPrimaryLiteral {
    data01: vec![9223372036854775808],
    num_bits: 64,
    signed: false,
};

let b: SvPrimaryLiteral = a.add_usize(9223372036854775808);

let exp = SvPrimaryLiteral {
    data01: vec![1, 0],
    num_bits: 65,
    signed: false,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);