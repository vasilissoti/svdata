let a = SvPrimaryLiteralIntegral {
    data01: vec![4611686018427387904],
    dataXZ: None,
    size: 63,
    signed: false,
};

let b = SvPrimaryLiteralIntegral {
    data01: vec![4611686018427387904],
    dataXZ: None,
    size: 63,
    signed: false,
};

let c: SvPrimaryLiteralIntegral = a.add_primlit(b.clone());

let exp = SvPrimaryLiteralIntegral {
    data01: vec![9223372036854775808],
    dataXZ: None,
    size: 64,
    signed: false,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);