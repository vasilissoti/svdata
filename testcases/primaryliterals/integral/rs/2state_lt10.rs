let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 3],
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

let c = a.lt(b.clone());
let zero = SvPrimaryLiteralIntegral {
    data_01: vec![0],
    data_xz: None,
    size: 1,
    signed: false,
};

assert_eq!(c, zero);

let actual_string = format!("{}", c);