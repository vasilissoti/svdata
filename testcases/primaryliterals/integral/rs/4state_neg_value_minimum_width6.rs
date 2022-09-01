let mut a = SvPrimaryLiteralIntegral {
    data_01: vec![0, 0],
    data_xz: Some(vec![0, 0]),
    size: 128,
    signed: false,
};

a._minimum_width();

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0],
    data_xz: Some(vec![0]),
    size: 1,
    signed: false,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);