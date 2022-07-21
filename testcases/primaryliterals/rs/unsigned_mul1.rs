let a = SvPrimaryLiteral {
    data01: vec![8],
    size: 4,
    signed: false,
};

let b = SvPrimaryLiteral {
    data01: vec![9223372036854775808],
    size: 64,
    signed: false,
};

let c: SvPrimaryLiteral = a.mul(b.clone());

let exp = SvPrimaryLiteral {
    data01: vec![4, 0],
    size: 67,
    signed: false,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);