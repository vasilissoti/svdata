let mut a = SvPrimaryLiteralIntegral {
    data_01: vec![0, 0],
    data_xz: None,
    size: 128,
    signed: true,
};

let actual_string = format!("{}", a.is_zero());