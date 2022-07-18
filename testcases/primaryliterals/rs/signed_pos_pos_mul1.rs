let a = SvPrimaryLiteral {
    data01: vec![0, 9223372036854775808],
    num_bits: 64,
    signed: true,
};

let b = SvPrimaryLiteral {
    data01: vec![4],
    num_bits: 4,
    signed: true,
};

let c: SvPrimaryLiteral = a.mul(b.clone());

let actual_string = format!("{}", c);