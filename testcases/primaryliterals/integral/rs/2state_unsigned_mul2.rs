let a = SvPrimaryLiteralIntegral {
    data_01: vec![1, 9223372036854775808, 9223372036854775808],
    data_xz: None,
    size: 192,
    signed: false,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![16],
    data_xz: None,
    size: 5,
    signed: false,
};

let c: SvPrimaryLiteralIntegral = a * b;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![16, 0, 8, 8],
    data_xz: None,
    size: 196,
    signed: false,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);