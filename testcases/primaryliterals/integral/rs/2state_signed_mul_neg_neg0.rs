let a = SvPrimaryLiteralIntegral {
    data_01: vec![3],
    data_xz: None,
    size: 2,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![4],
    data_xz: None,
    size: 3,
    signed: true,
};

let c: SvPrimaryLiteralIntegral = a * b;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![4],
    data_xz: None,
    size: 4,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);