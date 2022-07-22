let mut a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808],
    data_xz: None,
    size: 64,
    signed: false,
};

let mut b = SvPrimaryLiteralIntegral {
    data_01: vec![0, 9223372036854775808],
    data_xz: None,
    size: 65,
    signed: true,
};

a._primlit_vec_elmnt_match(&mut b);

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0, 9223372036854775808],
    data_xz: None,
    size: 64,
    signed: false,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);