let mut a = SvPrimaryLiteral {
    data01: vec![0, 1],
    size: 128,
    signed: false,
};

a._truncate(1);

let exp = SvPrimaryLiteral {
    data01: vec![1],
    size: 1,
    signed: false,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);