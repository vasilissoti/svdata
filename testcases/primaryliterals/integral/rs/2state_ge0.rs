let a = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904],
    data_xz: None,
    size: 63,
    signed: false,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808],
    data_xz: None,
    size: 64,
    signed: false,
};

let c = a.ge(b);

assert_eq!(c, logic1b_0());

let actual_string = format!("{}", c);