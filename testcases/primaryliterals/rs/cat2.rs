let a = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    num_bits: 64,
    signed: true,
};

let b = SvPrimaryLiteral {
    data01: vec![3],
    num_bits: 4,
    signed: true,
};

let c: SvPrimaryLiteral = a.cat(b.clone());

let actual_string = format!("{}", c);