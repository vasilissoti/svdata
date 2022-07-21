let a = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    size: 63,
    signed: true,
};

let b = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    size: 64,
    signed: true,
};

let c: SvPrimaryLiteral = a.add_primlit(b.clone());

let exp = SvPrimaryLiteral {
    data01: vec![0],
    size: 1,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);