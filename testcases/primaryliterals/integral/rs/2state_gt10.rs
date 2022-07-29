let a = SvPrimaryLiteralIntegral {
    data_01: vec![3, 9223372036854775808],
    data_xz: None,
    size: 66,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808],
    data_xz: None,
    size: 64,
    signed: true,
};

let c: bool = a > b;

assert_eq!(c, false);

let actual_string = format!("{}", c);