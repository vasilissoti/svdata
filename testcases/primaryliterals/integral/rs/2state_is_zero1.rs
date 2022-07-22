let mut a = SvPrimaryLiteralIntegral {
    data_01: vec![0],
    data_xz: None,
    size: 1,
    signed: true,
};

let actual_string = format!("{}", a.is_zero());