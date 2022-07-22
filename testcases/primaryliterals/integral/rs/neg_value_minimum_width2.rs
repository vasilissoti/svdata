let mut a = SvPrimaryLiteralIntegral {
    data01: vec![1, 9223372036854775808],
    size: 65,
    signed: true,
};

a._minimum_width();

let exp = SvPrimaryLiteralIntegral {
    data01: vec![9223372036854775808],
    size: 64,
    signed: true,
};

assert_eq!(a, exp);


let actual_string = format!("{}", a);