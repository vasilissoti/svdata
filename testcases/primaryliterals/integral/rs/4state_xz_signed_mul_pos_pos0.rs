let a = SvPrimaryLiteralIntegral {
    data_01: vec![0],
    data_xz: Some(vec![1]),
    size: 3,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![4],
    data_xz: Some(vec![0]),
    size: 4,
    signed: true,
};

let c: SvPrimaryLiteralIntegral = a.mul(b.clone());

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0],
    data_xz: Some(vec![127]),
    size: 7,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);