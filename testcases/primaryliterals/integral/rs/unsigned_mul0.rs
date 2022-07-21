let a = SvPrimaryLiteralIntegral {
    data01: vec![3],
    dataXZ: None,
    size: 2,
    signed: false,
};

let b = SvPrimaryLiteralIntegral {
    data01: vec![4],
    dataXZ: None,
    size: 3,
    signed: false,
};

let c: SvPrimaryLiteralIntegral = a.mul(b.clone());

let exp = SvPrimaryLiteralIntegral {
    data01: vec![12],
    dataXZ: None,
    size: 4,
    signed: false,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);