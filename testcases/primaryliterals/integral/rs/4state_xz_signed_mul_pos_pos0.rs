let a = SvPrimaryLiteralIntegral {
    data_01: vec![0],
    data_xz: Some(vec![1]),
    size: 2,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![16],
    data_xz: Some(vec![0]),
    size: 5,
    signed: true,
};

let c: SvPrimaryLiteralIntegral = a * b;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0],
    data_xz: Some(vec![127]),
    size: 7,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);