let a = SvPrimaryLiteralIntegral {
    data01: vec![0, 9223372036854775808],
    size: 65,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a.neg();

let exp = SvPrimaryLiteralIntegral {
    data01: vec![9223372036854775808],
    size: 64,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);