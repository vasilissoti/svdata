let a = SvPrimaryLiteral {
    data01: vec![9223372036854775808],
    num_bits: 64,
    signed: true,
};

let b = SvPrimaryLiteral {
    data01: vec![4],
    num_bits: 3,
    signed: true,
};

let c: SvPrimaryLiteral = a.mul(b.clone());

let exp = SvPrimaryLiteral {
    data01: vec![2, 0],
    num_bits: 67,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);