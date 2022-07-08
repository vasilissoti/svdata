let mut a = SvPrimaryLiteral {
    data01: vec![1, 9223372036854775808],
    num_bits: 66,
    signed: true,
};

let actual_string = format!("{}", a._signed_is_negative());