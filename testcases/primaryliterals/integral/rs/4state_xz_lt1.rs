let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808],
    data_xz: Some(vec![9223372036854775808]),
    size: 64,
    signed: false,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904],
    data_xz: Some(vec![0]),
    size: 63,
    signed: false,
};

let c = a.lt(b.clone());

assert_eq!(c, logic1b_x());

let actual_string = format!("{}", c);