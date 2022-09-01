let a = SvPrimaryLiteralIntegral {
    data_01: vec![0, 9223372036854775808],
    data_xz: Some(vec!{0, 9223372036854775808}),
    size: 65,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![0, 9223372036854775808],
    data_xz: Some(vec!{0, 0}),
    size: 66,
    signed: true,
};

let c = a.logical_eq(b.clone());

assert_eq!(c, logic1b_x());

let actual_string = format!("{}", c);