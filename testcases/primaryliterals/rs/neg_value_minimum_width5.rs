let mut a = SvPrimaryLiteral {
    data01: vec![0, 3],
    num_bits: 128,
    signed: false,
};

a._minimum_width();

let exp = SvPrimaryLiteral {
    data01: vec![3],
    num_bits: 2,
    signed: false,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);