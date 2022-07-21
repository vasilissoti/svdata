let a = SvPrimaryLiteral {
    data01: vec![0, 9223372036854775809],
    size: 65,
    signed: true,
};

let b = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    size: 63,
    signed: true,
};

let c: SvPrimaryLiteral = a.cat(b.clone());

let exp = SvPrimaryLiteral {
    data01: vec![4611686018427387904, 13835058055282163712],
    size: 128,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);