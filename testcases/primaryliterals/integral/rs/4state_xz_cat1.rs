let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 0],
    data_xz: Some(vec![9223372036854775808, 0]),
    size: 65,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![0],
    data_xz: Some(vec![4611686018427387904]),
    size: 63,
    signed: true,
};

let c: SvPrimaryLiteralIntegral = a.cat(b.clone());

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0, 4611686018427387904],
    data_xz: Some(vec![4611686018427387904, 4611686018427387904]),
    size: 128,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);