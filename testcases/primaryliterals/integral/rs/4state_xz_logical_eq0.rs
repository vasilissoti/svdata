let a = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904],
    data_xz: Some(vec!{4611686018427387904}),
    size: 64,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808],
    data_xz: Some(vec!{9223372036854775808}),
    size: 64,
    signed: true,
};

let c = a.logical_eq(b);

assert_eq!(c, logic1b_x());

let actual_string = format!("{}", c);