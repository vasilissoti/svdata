let a = SvPrimaryLiteralIntegral {
    data01: vec![9223372036854775808],
    dataXZ: None,
    size: 64,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data01: vec![4611686018427387904],
    dataXZ: None,
    size: 64,
    signed: true,
};

let c: bool = a == b;

let actual_string = format!("{}", c);