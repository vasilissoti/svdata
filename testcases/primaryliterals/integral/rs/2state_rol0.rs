let a = SvPrimaryLiteralIntegral {
    data_01: vec![1, 9223372036854775808],
    data_xz: None,
    size: 65,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a.rol(2);

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0, 3],
    data_xz: None,
    size: 65,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);