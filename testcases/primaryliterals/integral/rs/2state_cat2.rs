let a = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904],
    data_xz: None,
    size: 64,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![3],
    data_xz: None,
    size: 4,
    signed: true,
};

let c: SvPrimaryLiteralIntegral = a.cat(b.clone());

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![4, 3],
    data_xz: None,
    size: 68,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);