let a = SvPrimaryLiteralIntegral {
    data_01: vec![7],
    data_xz: Some(vec![3]),
    size: 3,
    signed: false,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![15],
    data_xz: Some(vec![0]),
    size: 5,
    signed: true,
};

let c: SvPrimaryLiteralIntegral = a.add_primlit(b.clone());

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0],
    data_xz: Some(vec![63]),
    size: 6,
    signed: false,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);