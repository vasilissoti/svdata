let mut a = SvPrimaryLiteralIntegral {
    data_01: vec![65533],
    data_xz: None,
    size: 16,
    signed: true,
};

a._minimum_width();

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![5],
    data_xz: None,
    size: 3,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);