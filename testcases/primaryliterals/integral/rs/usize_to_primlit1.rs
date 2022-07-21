let a: SvPrimaryLiteralIntegral = usize_to_primlit(9223372036854775808);

let exp = SvPrimaryLiteralIntegral {
    data01: vec![9223372036854775808],
    size: 64,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);