let a = SvPrimaryLiteral {
    data01: vec![3, 9223372036854775809],
    size: 65,
    signed: true,
};

let b: SvPrimaryLiteral = a.lsr(2);

let exp = SvPrimaryLiteral {
    data01: vec![0, 16140901064495857664],
    size: 65,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);