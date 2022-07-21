let mut a = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    size: 64,
    signed: true,
};

let actual_string = format!("{}", a.is_negative());