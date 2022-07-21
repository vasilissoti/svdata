let a = SvPrimaryLiteralIntegral {
    data01: vec![1, 9223372036854775808],
    size: 65,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a.rol(2);

let exp = SvPrimaryLiteralIntegral {
    data01: vec![0, 3],
    size: 65,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);