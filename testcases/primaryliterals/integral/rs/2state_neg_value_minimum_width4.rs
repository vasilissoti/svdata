let mut a = SvPrimaryLiteralIntegral {
    data_01: vec![0, 0],
    data_xz: None,
    size: 128,
    signed: true,
};

a._minimum_width();

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0],
    data_xz: None,
    size: 1,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);