let a = SvPrimaryLiteralIntegral {
    data01: vec![9223372036854775808, 9223372036854775808],
    dataXZ: None,
    size: 128,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data01: vec![9223372036854775808],
    dataXZ: None,
    size: 64,
    signed: true,
};

let c: SvPrimaryLiteralIntegral = a.add_primlit(b.clone());

let exp = SvPrimaryLiteralIntegral {
    data01: vec![9223372036854775808, 0],
    dataXZ: None,
    size: 128,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);