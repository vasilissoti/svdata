let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 0],
    data_xz: Some(vec![9223372036854775808, 0]),
    size: 65,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a >> 1;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904, 0],
    data_xz: Some(vec![4611686018427387904, 0]),
    size: 65,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);