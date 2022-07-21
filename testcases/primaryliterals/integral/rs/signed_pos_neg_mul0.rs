let a = SvPrimaryLiteralIntegral {
    data01: vec![3],
    dataXZ: None,
    size: 3,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data01: vec![4],
    dataXZ: None,
    size: 3,
    signed: true,
};

let c: SvPrimaryLiteralIntegral = a.mul(b.clone());

let exp = SvPrimaryLiteralIntegral {
    data01: vec![20],
    dataXZ: None,
    size: 5,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);