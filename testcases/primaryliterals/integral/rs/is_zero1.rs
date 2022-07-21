let mut a = SvPrimaryLiteralIntegral {
    data01: vec![0],
    dataXZ: None,
    size: 1,
    signed: true,
};

let actual_string = format!("{}", a.is_zero());