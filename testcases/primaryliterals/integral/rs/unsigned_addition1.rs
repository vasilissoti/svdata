let a = SvPrimaryLiteralIntegral {
    data01: vec![9223372036854775808],
    dataXZ: None,
    size: 64,
    signed: false,
};

let b = SvPrimaryLiteralIntegral {
    data01: vec![9223372036854775808],
    dataXZ: None,
    size: 64,
    signed: false,
};

let c: SvPrimaryLiteralIntegral = a.add_primlit(b.clone());

let exp = SvPrimaryLiteralIntegral {
    data01: vec![1, 0],
    dataXZ: None,
    size: 65,
    signed: false,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);