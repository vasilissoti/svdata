let mut a = SvPrimaryLiteral {
    data01: vec![0],
    num_bits: 1,
    signed: true,
};

let actual_string = format!("{}", a._is_zero());