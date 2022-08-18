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

let c = a.le(b.clone());
let unknown = SvPrimaryLiteralIntegral {
    data_01: vec![0],
    data_xz: Some(vec![1]),
    size: 1,
    signed: false,
};

assert_eq!(c, unknown);

let actual_string = format!("{}", c);