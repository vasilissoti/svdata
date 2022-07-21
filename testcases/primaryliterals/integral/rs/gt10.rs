let a = SvPrimaryLiteralIntegral {
    data01: vec![3, 9223372036854775808],
    dataXZ: None,
    size: 66,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data01: vec![9223372036854775808],
    dataXZ: None,
    size: 64,
    signed: true,
};

let c: bool = a > b;

let actual_string = format!("{}", c);