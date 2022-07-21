let a = SvPrimaryLiteral {
    data01: vec![9223372036854775808],
    size: 64,
    signed: false,
};

let b = SvPrimaryLiteral {
    data01: vec![4],
    size: 3,
    signed: true,
};

let c: SvPrimaryLiteral = a.mul(b.clone());

let exp = SvPrimaryLiteral {
    data01: vec![2, 0],
    size: 66,
    signed: false,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);