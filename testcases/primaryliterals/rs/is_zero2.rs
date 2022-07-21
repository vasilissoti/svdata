let mut a = SvPrimaryLiteral {
    data01: vec![0, 0],
    size: 128,
    signed: true,
};

let actual_string = format!("{}", a.is_zero());