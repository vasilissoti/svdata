let mut a = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904],
    data_xz: None,
    size: 64,
    signed: true,
};

let b: bool = a.is_negative();

assert_eq!(b, false);

let actual_string = format!("{}", b);