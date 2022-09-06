let a = SvPrimaryLiteralIntegral {
    data_01: vec![0, 0],
    data_xz: None,
    size: 128,
    signed: true,
};

let b: bool = a.is_zero();

assert_eq!(b, true);

let actual_string = format!("{}", b);