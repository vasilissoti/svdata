let mut a = SvPrimaryLiteral {
    data01: vec![0, 1],
    num_bits: 128,
    signed: false,
};

a._truncate(1);

let actual_string = format!("{}", a);