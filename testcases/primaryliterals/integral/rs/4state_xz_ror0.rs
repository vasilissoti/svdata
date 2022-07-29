let a = SvPrimaryLiteralIntegral {
    data_01: vec![3, 9223372036854775809],
    data_xz: Some(vec![0, 1]),
    size: 66,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a.ror(2);

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![1, 16140901064495857664],
    data_xz: Some(vec![1, 0]),
    size: 66,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);