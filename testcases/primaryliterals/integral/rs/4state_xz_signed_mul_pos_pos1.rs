let a = SvPrimaryLiteralIntegral {
    data_01: vec![0, 9223372036854775808],
    data_xz: Some(vec![1, 0]),
    size: 64,
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
    data_01: vec![0, 0],
    data_xz: Some(vec![15, 18446744073709551615]),
    size: 68,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);