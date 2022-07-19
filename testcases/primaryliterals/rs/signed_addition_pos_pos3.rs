let a = SvPrimaryLiteral {
    data01: vec![4611686018427387904, 4611686018427387904],
    num_bits: 128,
    signed: true,
};

let b = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    num_bits: 64,
    signed: true,
};

let c: SvPrimaryLiteral = a.add_primlit(b.clone());

let exp = SvPrimaryLiteral {
    data01: vec![4611686018427387904, 9223372036854775808],
    num_bits: 128,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);