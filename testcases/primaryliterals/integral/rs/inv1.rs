let a = SvPrimaryLiteralIntegral {
    data01: vec![4611686018427387904, 4611686018427387904],
    size: 127,
    signed: false,
};

let b: SvPrimaryLiteralIntegral = a.inv();

let exp = SvPrimaryLiteralIntegral {
    data01: vec![4611686018427387903, 13835058055282163711],
    size: 127,
    signed: false,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);