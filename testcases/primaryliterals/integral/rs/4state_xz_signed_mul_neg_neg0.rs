let a = SvPrimaryLiteralIntegral {
    data_01: vec![3],
    data_xz: Some(vec![0]),
    size: 2,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![8],
    data_xz: Some(vec![4]),
    size: 4,
    signed: true,
};

let c: SvPrimaryLiteralIntegral = a * b;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0],
    data_xz: Some(vec![63]),
    size: 6,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);