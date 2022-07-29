let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808],
    data_xz: None,
    size: 64,
    signed: false,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![4],
    data_xz: None,
    size: 3,
    signed: true,
};

let c: SvPrimaryLiteralIntegral = a.mul(b.clone());

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![2, 0],
    data_xz: None,
    size: 66,
    signed: false,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);