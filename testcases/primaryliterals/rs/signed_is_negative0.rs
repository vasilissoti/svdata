let mut a = SvPrimaryLiteral {
    data01: vec![9223372036854775808],
    size: 64,
    signed: true,
};

let actual_string = format!("{}", a.is_negative());