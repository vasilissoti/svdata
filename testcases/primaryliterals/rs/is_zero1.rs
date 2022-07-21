let mut a = SvPrimaryLiteral {
    data01: vec![0],
    size: 1,
    signed: true,
};

let actual_string = format!("{}", a.is_zero());