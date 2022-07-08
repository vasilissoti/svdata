let mut a = SvPrimaryLiteral {
    data01: vec![0, 0],
    num_bits: 128,
    signed: true,
};

let actual_string = format!("{}", a._is_zero());