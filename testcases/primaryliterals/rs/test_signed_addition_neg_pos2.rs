let mut a = SvPrimaryLiteral {
    data01: vec![9223372036854775808],
    num_bits: 64,
    signed: true,
};

a.usize_add(4611686018427387904);

let actual_string = format!("{}", a);