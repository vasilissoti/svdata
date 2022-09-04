let a = SvPrimaryLiteralIntegral {
    data_01: vec![0, 9223372036854775808],
    data_xz: Some(vec![9223372036854775808, 9223372036854775808]),
    size: 128,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a >> 1;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0, 4611686018427387904],
    data_xz: Some(vec![4611686018427387904, 4611686018427387904]),
    size: 128,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);