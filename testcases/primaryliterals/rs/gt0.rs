let a = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    num_bits: 63,
    signed: false,
};

let b = SvPrimaryLiteral {
    data01: vec![9223372036854775808],
    num_bits: 64,
    signed: false,
};

let c: bool = a.gt(b.clone());

let actual_string = format!("{}", c);