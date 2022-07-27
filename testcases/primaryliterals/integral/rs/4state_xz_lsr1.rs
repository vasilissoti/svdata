let a = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904],
    data_xz: Some(vec![4611686018427387904]),
    size: 64,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a.lsr(1);

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![2305843009213693952],
    data_xz: Some(vec![2305843009213693952]),
    size: 64,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);