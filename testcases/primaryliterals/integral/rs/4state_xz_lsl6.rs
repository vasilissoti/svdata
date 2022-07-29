let a = SvPrimaryLiteralIntegral {
    data_01: vec![0],
    data_xz: Some(vec![9223372036854775808]),
    size: 64,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a.lsl(2);

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0, 0],
    data_xz: Some(vec![2, 0]),
    size: 66,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);