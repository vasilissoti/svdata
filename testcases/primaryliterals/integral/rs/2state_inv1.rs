let a = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904, 4611686018427387904],
    data_xz: None,
    size: 127,
    signed: false,
};

let b: SvPrimaryLiteralIntegral = !a;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387903, 13835058055282163711],
    data_xz: None,
    size: 127,
    signed: false,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);