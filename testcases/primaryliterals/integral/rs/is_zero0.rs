let mut a = SvPrimaryLiteralIntegral {
    data01: vec![1, 0],
    dataXZ: None,
    size: 65,
    signed: true,
};

let actual_string = format!("{}", a.is_zero());