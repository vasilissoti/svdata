let a = SvPrimaryLiteralIntegral {
    data_01: vec![0, 9223372036854775808, 1],
    data_xz: Some(vec![9223372036854775808, 0, 0]),
    size: 192,
    signed: false,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![16],
    data_xz: Some(vec![0]),
    size: 5,
    signed: false,
};

let c: SvPrimaryLiteralIntegral = a.mul(b.clone());

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0, 0, 0, 0],
    data_xz: Some(vec![31, 18446744073709551615, 18446744073709551615, 18446744073709551615]),
    size: 197,
    signed: false,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);