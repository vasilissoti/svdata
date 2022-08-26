let mut a = SvPrimaryLiteralIntegral {
    data_01: vec![1, 9223372036854775808],
    data_xz: Some(vec![1, 0]),
    size: 65,
    signed: true,
};

let mut b = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904],
    data_xz: Some(vec![0]),
    size: 64,
    signed: true,
};

a._matched_sign_extend(&mut b);

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![18446744073709551615, 9223372036854775808],
    data_xz: Some(vec![18446744073709551615, 0]),
    size: 128,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);