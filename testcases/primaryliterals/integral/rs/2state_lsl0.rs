let a = SvPrimaryLiteralIntegral {
    data_01: vec![0, 9223372036854775808],
    data_xz: None,
    size: 65,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a << 1;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![1, 0],
    data_xz: None,
    size: 66,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);