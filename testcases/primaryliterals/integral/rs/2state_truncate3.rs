let mut a = SvPrimaryLiteralIntegral {
    data_01: vec![0, 1],
    data_xz: None,
    size: 128,
    signed: false,
};

a._truncate(1);

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![1],
    data_xz: None,
    size: 1,
    signed: false,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);