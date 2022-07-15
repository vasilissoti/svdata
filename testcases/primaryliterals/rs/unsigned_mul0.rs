let a = SvPrimaryLiteral {
    data01: vec![3],
    num_bits: 2,
    signed: false,
};

let b = SvPrimaryLiteral {
    data01: vec![4],
    num_bits: 3,
    signed: false,
};

let c: SvPrimaryLiteral = a.mul(b.clone());

let actual_string = format!("{}", c);