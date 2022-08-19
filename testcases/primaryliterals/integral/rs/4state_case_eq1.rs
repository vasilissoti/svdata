let a = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904],
    data_xz: Some(vec!{0}),
    size: 63,
    signed: false,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904],
    data_xz: Some(vec!{0}),
    size: 64,
    signed: false,
};

let c = a.case_eq(b.clone());

assert_eq!(c, bit1b_1());

let actual_string = format!("{}", c);