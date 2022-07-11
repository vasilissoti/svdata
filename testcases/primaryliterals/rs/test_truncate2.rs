let mut a = SvPrimaryLiteral {
    data01: vec![9223372036854775809, 9223372036854775808],
    num_bits: 128,
    signed: false,
};

a._truncate(69);

let actual_string = format!("{}", a);