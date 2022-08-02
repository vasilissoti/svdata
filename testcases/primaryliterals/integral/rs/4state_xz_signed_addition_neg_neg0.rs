let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808],
    data_xz: Some(vec![9223372036854775808]),
    size: 64,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a.add_usize(9223372036854775808);

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0, 0],
    data_xz: Some(vec![1, 18446744073709551615]),
    size: 65,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);