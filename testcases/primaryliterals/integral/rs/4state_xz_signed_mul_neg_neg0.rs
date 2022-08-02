let a = SvPrimaryLiteralIntegral {
    data_01: vec![3],
    data_xz: Some(vec![0]),
    size: 2,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![4],
    data_xz: Some(vec![4]),
    size: 3,
    signed: true,
};

let c: SvPrimaryLiteralIntegral = a.mul(b.clone());

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0],
    data_xz: Some(vec![31]),
    size: 5,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);