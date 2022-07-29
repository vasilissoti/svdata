let mut a = SvPrimaryLiteralIntegral {
    data_01: vec![1, 9223372036854775808],
    data_xz: None,
    size: 66,
    signed: true,
};

let b: bool = a.is_negative();

assert_eq!(b, false);

let actual_string = format!("{}", b);