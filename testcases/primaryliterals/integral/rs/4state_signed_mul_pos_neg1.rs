let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 0],
    data_xz: Some(vec![0, 0]),
    size: 65,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![4],
    data_xz: Some(vec![0]),
    size: 3,
    signed: true,
};

let c: SvPrimaryLiteralIntegral = a * b;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0, 14],
    data_xz: Some(vec![0, 0]),
    size: 68,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);