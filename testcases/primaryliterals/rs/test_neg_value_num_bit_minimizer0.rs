let mut a = SvPrimaryLiteral {
    data01: vec![65533],
    num_bits: 16,
    signed: true,
};

a._neg_value_num_bit_minimizer();

let actual_string = format!("{}", a);