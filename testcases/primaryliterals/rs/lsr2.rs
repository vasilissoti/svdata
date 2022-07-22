let a = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    num_bits: 64,
    signed: true,
};

let b: SvPrimaryLiteral = a.lsr(4);

let exp = SvPrimaryLiteral {
    data01: vec![288230376151711744],
    num_bits: 64,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);