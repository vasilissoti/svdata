let mut a = SvPrimaryLiteralIntegral {
    data01: vec![1, 9223372036854775808],
    size: 65,
    signed: true,
};

let mut b = SvPrimaryLiteralIntegral {
    data01: vec![4611686018427387904],
    size: 64,
    signed: true,
};

a._matched_sign_extension(&mut b);

let exp = SvPrimaryLiteralIntegral {
    data01: vec![18446744073709551615, 9223372036854775808],
    size: 128,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);