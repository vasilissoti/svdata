let a = SvPrimaryLiteral {
    data01: vec![9223372036854775808, 9223372036854775808],
    size: 128,
    signed: true,
};

let b = SvPrimaryLiteral {
    data01: vec![9223372036854775808],
    size: 64,
    signed: true,
};

let c: SvPrimaryLiteral = a.add_primlit(b.clone());

let exp = SvPrimaryLiteral {
    data01: vec![9223372036854775808, 0],
    size: 128,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);