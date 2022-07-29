let a = SvPrimaryLiteralIntegral {
    data_01: vec![0, 9223372036854775808],
    data_xz: Some(vec![0, 9223372036854775808]),
    size: 65,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904],
    data_xz: Some(vec![0]),
    size: 64,
    signed: true,
};

let c: bool = a < b;

let actual_string = format!("{}", c);