let a: SvPrimaryLiteralIntegral = usize_to_primlit(9223372036854775808);

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808],
    data_xz: None,
    size: 64,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);