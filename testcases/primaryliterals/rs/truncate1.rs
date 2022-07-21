let mut a = SvPrimaryLiteralIntegral {
    data01: vec![9223372036854775808, 4611686018427387905],
    size: 128,
    signed: true,
};

a._truncate(5);

let exp = SvPrimaryLiteralIntegral {
    data01: vec![1],
    size: 5,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);