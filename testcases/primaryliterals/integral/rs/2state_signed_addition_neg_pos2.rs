let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808],
    data_xz: None,
    size: 64,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a + 4611686018427387904;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![1, 13835058055282163712],
    data_xz: None,
    size: 65,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);