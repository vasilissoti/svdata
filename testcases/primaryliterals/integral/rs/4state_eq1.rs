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

let c: bool = a == b;

assert_eq!(c, true);

let actual_string = format!("{}", c);