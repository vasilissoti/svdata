let mut a = SvPrimaryLiteral {
    data01: vec![1, 9223372036854775808],
    num_bits: 128,
    signed: true,
};

a._minimum_width();

let actual_string = format!("{}", a);