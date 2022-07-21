let a = SvPrimaryLiteral {
    data01: vec![9223372036854775808, 9223372036854775808],
    size: 128,
    signed: false,
};

let b: SvPrimaryLiteral = a.add_usize(9223372036854775808);

let exp = SvPrimaryLiteral {
    data01: vec![9223372036854775809, 0],
    size: 128,
    signed: false,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);