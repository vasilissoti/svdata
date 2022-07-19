let mut a = SvPrimaryLiteral {
    data01: vec![65533],
    num_bits: 16,
    signed: true,
};

a._minimum_width();

let exp = SvPrimaryLiteral {
    data01: vec![5],
    num_bits: 3,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);