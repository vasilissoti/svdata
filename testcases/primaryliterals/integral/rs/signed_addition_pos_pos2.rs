let a = SvPrimaryLiteralIntegral {
    data01: vec![4611686018427387904, 4611686018427387904],
    dataXZ: None,
    size: 128,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a.add_usize(4611686018427387904);

let exp = SvPrimaryLiteralIntegral {
    data01: vec![4611686018427387904, 9223372036854775808],
    dataXZ: None,
    size: 128,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);