let mut a = SvPrimaryLiteral {
    data01: vec![0, 9223372036854775808],
    num_bits: 192,
    signed: false,
};

a.usize_add(9223372036854775808);

let actual_string = format!("{}", a);