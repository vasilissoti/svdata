let a = SvPrimaryLiteralIntegral {
    data01: vec![4611686018427387904],
    dataXZ: None,
    size: 64,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data01: vec![4611686018427387904],
    dataXZ: None,
    size: 64,
    signed: true,
};

let c: SvPrimaryLiteralIntegral = a.add_primlit(b.clone());

let exp = SvPrimaryLiteralIntegral {
    data01: vec![0, 9223372036854775808],
    dataXZ: None,
    size: 65,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);