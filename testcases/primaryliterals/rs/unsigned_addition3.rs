let a = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    num_bits: 63,
    signed: false,
};

let b = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    num_bits: 63,
    signed: false,
};

let c: SvPrimaryLiteral = a.add_primlit(b.clone());

let exp = SvPrimaryLiteral {
    data01: vec![9223372036854775808],
    num_bits: 64,
    signed: false,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);