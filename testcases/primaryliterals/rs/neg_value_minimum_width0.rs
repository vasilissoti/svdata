let mut a = SvPrimaryLiteral {
    data01: vec![65533],
    num_bits: 16,
    signed: true,
};

a._minimum_width();

let actual_string = format!("{}", a);