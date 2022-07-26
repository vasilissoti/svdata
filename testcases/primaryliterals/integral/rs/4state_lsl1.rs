let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 9223372036854775808],
    data_xz: Some(vec![0, 0]),
    size: 128,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a.lsl(2);

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![2, 2, 0],
    data_xz: Some(vec![0, 0, 0]),
    size: 130,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);