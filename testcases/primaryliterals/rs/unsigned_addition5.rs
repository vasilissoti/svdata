let a = SvPrimaryLiteral {
    data01: vec![9223372036854775808, 9223372036854775808],
    num_bits: 128,
    signed: false,
};

let b = SvPrimaryLiteral {
    data01: vec![9223372036854775808],
    num_bits: 64,
    signed: false,
};

let c: SvPrimaryLiteral = a.add_primlit(b.clone());

let actual_string = format!("{}", c);