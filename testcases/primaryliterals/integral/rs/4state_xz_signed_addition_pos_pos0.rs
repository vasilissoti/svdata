let a = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904],
    data_xz: Some(vec![4611686018427387904]),
    size: 64,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a.add_usize(4611686018427387904);

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0, 0],
    data_xz: Some(vec![1, 18446744073709551615]),
    size: 65,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);