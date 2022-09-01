let a = SvPrimaryLiteralIntegral {
    data_01: vec![0],
    data_xz: Some(vec![4611686018427387904]),
    size: 63,
    signed: false,
};

let b: SvPrimaryLiteralIntegral = a + 4611686018427387904;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0, 0],
    data_xz: Some(vec![1, 18446744073709551615]),
    size: 65,
    signed: false,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);