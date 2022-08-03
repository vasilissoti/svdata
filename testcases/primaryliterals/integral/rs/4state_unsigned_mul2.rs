let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 9223372036854775808, 1],
    data_xz: Some(vec![0, 0, 0]),
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
    data_01: vec![8, 8, 0, 16],
    data_xz: Some(vec![0, 0, 0, 0]),
    size: 196,
    signed: false,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);