let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808],
    data_xz: None,
    size: 64,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808],
    data_xz: None,
    size: 64,
    signed: false,
};

let c: SvPrimaryLiteralIntegral = a.add_primlit(b.clone());

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![1, 0],
    data_xz: None,
    size: 65,
    signed: false,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);