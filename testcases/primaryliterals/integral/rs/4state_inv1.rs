let a = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904, 4611686018427387904],
    data_xz: Some(vec![0, 0]),
    size: 127,
    signed: false,
};

let b: SvPrimaryLiteralIntegral = a.inv();

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![13835058055282163711, 4611686018427387903],
    data_xz: Some(vec![0, 0]),
    size: 127,
    signed: false,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);