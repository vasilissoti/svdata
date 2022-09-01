let a = SvPrimaryLiteralIntegral {
    data_01: vec![0],
    data_xz: Some(vec![8]),
    size: 4,
    signed: false,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808],
    data_xz: Some(vec![0]),
    size: 64,
    signed: false,
};

let c: SvPrimaryLiteralIntegral = a * b;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0, 0],
    data_xz: Some(vec![15, 18446744073709551615]),
    size: 68,
    signed: false,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);