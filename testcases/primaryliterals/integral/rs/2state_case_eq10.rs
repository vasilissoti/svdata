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

let c = a.case_eq(b);

assert_eq!(c, bit1b_1());

let actual_string = format!("{}", c);