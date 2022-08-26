let mut a = SvPrimaryLiteralIntegral {
    data_01: vec![3, 0],
    data_xz: None,
    size: 128,
    signed: false,
};

a._minimum_width();

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![3],
    data_xz: None,
    size: 2,
    signed: false,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);