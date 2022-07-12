let a = SvPrimaryLiteral {
    data01: vec![9223372036854775808, 9223372036854775808],
    num_bits: 128,
    signed: false,
};

let b: SvPrimaryLiteral = a.add_usize(9223372036854775808);

let actual_string = format!("{}", b);