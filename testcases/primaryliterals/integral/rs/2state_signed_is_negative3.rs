let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 1],
    data_xz: None,
    size: 66,
    signed: true,
};

let b: bool = a.is_negative();

assert_eq!(b, false);

let actual_string = format!("{}", b);