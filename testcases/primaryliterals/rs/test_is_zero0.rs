let mut a = SvPrimaryLiteral {
    data01: vec![1, 0],
    num_bits: 65,
    signed: true,
};

let actual_string = format!("{}", a._is_zero());