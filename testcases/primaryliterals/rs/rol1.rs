let a = SvPrimaryLiteral {
    data01: vec![9223372036854775808, 9223372036854775808],
    num_bits: 128,
    signed: true,
};

let b: SvPrimaryLiteral = a.rol(2);

let actual_string = format!("{}", b);