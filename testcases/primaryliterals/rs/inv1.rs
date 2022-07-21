let a = SvPrimaryLiteral {
    data01: vec![4611686018427387904, 4611686018427387904],
    size: 127,
    signed: false,
};

let b: SvPrimaryLiteral = a.inv();

let exp = SvPrimaryLiteral {
    data01: vec![4611686018427387903, 13835058055282163711],
    size: 127,
    signed: false,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);