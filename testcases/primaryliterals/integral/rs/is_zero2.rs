let mut a = SvPrimaryLiteralIntegral {
    data01: vec![0, 0],
    dataXZ: None,
    size: 128,
    signed: true,
};

let actual_string = format!("{}", a.is_zero());