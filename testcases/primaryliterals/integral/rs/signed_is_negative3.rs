let mut a = SvPrimaryLiteralIntegral {
    data01: vec![1, 9223372036854775808],
    dataXZ: None,
    size: 66,
    signed: true,
};

let actual_string = format!("{}", a.is_negative());