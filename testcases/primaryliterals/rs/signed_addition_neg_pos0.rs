let a = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    num_bits: 63,
    signed: true,
};

let b: SvPrimaryLiteral = a.add_usize(4611686018427387904);

let exp = SvPrimaryLiteral {
    data01: vec![0],
    num_bits: 1,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);