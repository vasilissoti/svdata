let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775809, 9223372036854775809],
    data_xz: Some(vec![0, 0]),
    size: 128,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a >> 2;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![6917529027641081856, 2305843009213693952],
    data_xz: Some(vec![0, 0]),
    size: 128,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);