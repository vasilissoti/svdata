let mut a = SvPrimaryLiteralIntegral {
    data_01: vec![0, 1],
    data_xz: None,
    size: 65,
    signed: true,
};

let b: bool = a.is_zero();

assert_eq!(b, false);

let actual_string = format!("{}", b);