let mut a = SvPrimaryLiteral {
    data01: vec![9223372036854775808],
    num_bits: 64,
    signed: true,
};

a._neg_value_num_bit_minimizer();

let actual_string = format!("{}", a);