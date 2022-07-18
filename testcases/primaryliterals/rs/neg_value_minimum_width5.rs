let mut a = SvPrimaryLiteral {
    data01: vec![0, 3],
    num_bits: 128,
    signed: false,
};

a._minimum_width();

let actual_string = format!("{}", a);