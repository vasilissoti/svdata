let mut a = SvPrimaryLiteral {
    data01: vec![9223372036854775808],
    size: 64,
    signed: false,
};

let mut b = SvPrimaryLiteral {
    data01: vec![0, 9223372036854775808],
    size: 65,
    signed: true,
};

a._primlit_vec_elmnt_match(&mut b);

let exp = SvPrimaryLiteral {
    data01: vec![0, 9223372036854775808],
    size: 64,
    signed: false,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);