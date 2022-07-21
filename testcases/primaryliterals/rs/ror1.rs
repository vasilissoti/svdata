let a = SvPrimaryLiteralIntegral {
    data01: vec![9223372036854775809, 9223372036854775809],
    size: 128,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a.ror(2);

let exp = SvPrimaryLiteralIntegral {
    data01: vec![6917529027641081856, 6917529027641081856],
    size: 128,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);