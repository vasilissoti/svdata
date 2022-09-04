let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 0],
    data_xz: Some(vec![0, 1]),
    size: 66,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![4],
    data_xz: Some(vec![0]),
    size: 4,
    signed: true,
};

let c: SvPrimaryLiteralIntegral = a * b;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0, 0],
    data_xz: Some(vec![18446744073709551615, 63]),
    size: 70,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);