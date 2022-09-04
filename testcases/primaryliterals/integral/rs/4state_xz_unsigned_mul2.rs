let a = SvPrimaryLiteralIntegral {
    data_01: vec![1, 9223372036854775808, 0],
    data_xz: Some(vec![0, 0, 9223372036854775808]),
    size: 192,
    signed: false,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![16],
    data_xz: Some(vec![0]),
    size: 5,
    signed: false,
};

let c: SvPrimaryLiteralIntegral = a * b;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0, 0, 0, 0],
    data_xz: Some(vec![18446744073709551615, 18446744073709551615, 18446744073709551615, 31]),
    size: 197,
    signed: false,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);