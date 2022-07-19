let mut a = SvPrimaryLiteral {
    data01: vec![9223372036854775808, 4611686018427387905],
    num_bits: 128,
    signed: true,
};

a._truncate(5);

let exp = SvPrimaryLiteral {
    data01: vec![1],
    num_bits: 5,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);