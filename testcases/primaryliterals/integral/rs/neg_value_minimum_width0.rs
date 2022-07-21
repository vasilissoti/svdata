let mut a = SvPrimaryLiteralIntegral {
    data01: vec![65533],
    dataXZ: None,
    size: 16,
    signed: true,
};

a._minimum_width();

let exp = SvPrimaryLiteralIntegral {
    data01: vec![5],
    dataXZ: None,
    size: 3,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);