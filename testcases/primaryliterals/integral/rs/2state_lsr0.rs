let a = SvPrimaryLiteralIntegral {
    data_01: vec![3, 9223372036854775809],
    data_xz: None,
    size: 65,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a >> 2;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0, 16140901064495857664],
    data_xz: None,
    size: 65,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);