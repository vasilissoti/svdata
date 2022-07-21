let a = SvPrimaryLiteralIntegral {
    data01: vec![9223372036854775808, 9223372036854775808, 1],
    dataXZ: None,
    size: 192,
    signed: false,
};

let b = SvPrimaryLiteralIntegral {
    data01: vec![16],
    dataXZ: None,
    size: 5,
    signed: false,
};

let c: SvPrimaryLiteralIntegral = a.mul(b.clone());

let exp = SvPrimaryLiteralIntegral {
    data01: vec![8, 8, 0, 16],
    dataXZ: None,
    size: 196,
    signed: false,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);