let a = SvPrimaryLiteralIntegral {
    data01: vec![0, 9223372036854775808],
    dataXZ: None,
    size: 64,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data01: vec![4],
    dataXZ: None,
    size: 4,
    signed: true,
};

let c: SvPrimaryLiteralIntegral = a.mul(b.clone());

let exp = SvPrimaryLiteralIntegral {
    data01: vec![2, 0],
    dataXZ: None,
    size: 66,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);