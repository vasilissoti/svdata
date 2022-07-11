let mut a = SvPrimaryLiteral {
    data01: vec![9223372036854775808, 4611686018427387905],
    num_bits: 128,
    signed: true,
};

a._truncate_size(5);

let actual_string = format!("{}", a);