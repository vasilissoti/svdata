let a = SvPrimaryLiteralIntegral {
    data01: vec![9223372036854775808, 9223372036854775808],
    size: 128,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a.rol(2);

let exp = SvPrimaryLiteralIntegral {
    data01: vec![2, 2],
    size: 128,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);