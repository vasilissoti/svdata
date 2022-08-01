let a = SvPrimaryLiteralIntegral {
    data_01: vec![0, 9223372036854775808],
    data_xz: Some(vec![0, 0]),
    size: 65,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![0],
    data_xz: Some(vec![1]),
    size: 3,
    signed: true,
};

let c: SvPrimaryLiteralIntegral = a.mul(b.clone());

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0, 0],
    data_xz: Some(vec![15, 18446744073709551615]),
    size: 68,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);