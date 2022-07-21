let a = SvPrimaryLiteral {
    data01: vec![4611686018427387904, 4611686018427387904],
    size: 128,
    signed: true,
};

let b: SvPrimaryLiteral = a.add_usize(4611686018427387904);

let exp = SvPrimaryLiteral {
    data01: vec![4611686018427387904, 9223372036854775808],
    size: 128,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);