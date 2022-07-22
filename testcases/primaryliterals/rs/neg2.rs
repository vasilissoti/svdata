let a = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    num_bits: 64,
    signed: true,
};

let b: SvPrimaryLiteral = a.neg();

let exp = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    num_bits: 63,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);