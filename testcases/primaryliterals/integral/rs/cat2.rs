let a = SvPrimaryLiteralIntegral {
    data01: vec![4611686018427387904],
    dataXZ: None,
    size: 64,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data01: vec![3],
    dataXZ: None,
    size: 4,
    signed: true,
};

let c: SvPrimaryLiteralIntegral = a.cat(b.clone());

let exp = SvPrimaryLiteralIntegral {
    data01: vec![4, 3],
    dataXZ: None,
    size: 68,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);