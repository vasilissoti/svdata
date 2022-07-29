let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 0],
    data_xz: Some(vec![9223372036854775808, 9223372036854775808]),
    size: 128,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a.lsr(1);

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904, 0],
    data_xz: Some(vec![4611686018427387904, 4611686018427387904]),
    size: 128,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);