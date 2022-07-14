let a = SvPrimaryLiteral {
    data01: vec![0, 9223372036854775808],
    num_bits: 65,
    signed: true,
};

let b = SvPrimaryLiteral {
    data01: vec![0, 9223372036854775808],
    num_bits: 66,
    signed: true,
};

let c: bool = a.lt(b.clone());

let actual_string = format!("{}", c);