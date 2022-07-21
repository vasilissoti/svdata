let a = SvPrimaryLiteralIntegral {
    data01: vec![4611686018427387904],
    dataXZ: None,
    size: 64,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a.lsr(4);

let exp = SvPrimaryLiteralIntegral {
    data01: vec![288230376151711744],
    dataXZ: None,
    size: 64,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);