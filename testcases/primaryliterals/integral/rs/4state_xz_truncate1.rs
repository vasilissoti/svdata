let mut a = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387905, 9223372036854775808],
    data_xz: Some(vec![4611686018427387905, 0]),
    size: 128,
    signed: true,
};

a._truncate(5);

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![1],
    data_xz: Some(vec![1]),
    size: 5,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);