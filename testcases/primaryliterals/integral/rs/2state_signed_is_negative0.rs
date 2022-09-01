let mut a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808],
    data_xz: None,
    size: 64,
    signed: true,
};

let b: bool = a.is_negative();

assert_eq!(b, true);

let actual_string = format!("{}", b);