let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775809, 0],
    data_xz: None,
    size: 65,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![4611686018427387904],
    data_xz: None,
    size: 63,
    signed: true,
};

let c: SvPrimaryLiteralIntegral = a.cat(b);

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![13835058055282163712, 4611686018427387904],
    data_xz: None,
    size: 128,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);