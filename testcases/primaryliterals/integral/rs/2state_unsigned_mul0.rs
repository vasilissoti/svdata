let a = SvPrimaryLiteralIntegral {
    data_01: vec![3],
    data_xz: None,
    size: 2,
    signed: false,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![4],
    data_xz: None,
    size: 3,
    signed: false,
};

let c: SvPrimaryLiteralIntegral = a * b;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![12],
    data_xz: None,
    size: 5,
    signed: false,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);