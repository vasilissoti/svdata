let a = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904, 4611686018427387904],
    data_xz: Some(vec![0, 1]),
    size: 127,
    signed: false,
};

let b: SvPrimaryLiteralIntegral = !a;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387903, 13835058055282163710],
    data_xz: Some(vec![0, 1]),
    size: 127,
    signed: false,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);