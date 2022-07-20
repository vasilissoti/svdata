let a = SvPrimaryLiteral {
    data01: vec![3, 9223372036854775808],
    num_bits: 66,
    signed: true,
};

let b = SvPrimaryLiteral {
    data01: vec![9223372036854775808],
    num_bits: 64,
    signed: true,
};

let c: bool = a < b;

let actual_string = format!("{}", c);