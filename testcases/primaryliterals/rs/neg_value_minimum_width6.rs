let mut a = SvPrimaryLiteralIntegral {
    data01: vec![0, 0],
    size: 128,
    signed: false,
};

a._minimum_width();

let exp = SvPrimaryLiteralIntegral {
    data01: vec![0],
    size: 1,
    signed: false,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);