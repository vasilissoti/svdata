let mut a = SvPrimaryLiteralIntegral {
    data_01: vec![1, 0],
    data_xz: None,
    size: 65,
    signed: true,
};

let actual_string = format!("{}", a.is_zero());