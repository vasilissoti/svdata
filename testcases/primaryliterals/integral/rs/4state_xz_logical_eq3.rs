let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 0],
    data_xz: Some(vec!{9223372036854775808, 0}),
    size: 65,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 0],
    data_xz: Some(vec!{9223372036854775808, 0}),
    size: 65,
    signed: true,
};

let c = a.logical_eq(b.clone());
let unknown = SvPrimaryLiteralIntegral {
    data_01: vec![0],
    data_xz: Some(vec![1]),
    size: 1,
    signed: false,
};

assert_eq!(c, unknown);

let actual_string = format!("{}", c);