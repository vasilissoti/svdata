let mut a = SvPrimaryLiteral {
    data01: vec![9223372036854775808],
    num_bits: 64,
    signed: true,
};

let mut b = SvPrimaryLiteral {
    data01: vec![0, 9223372036854775808],
    num_bits: 65,
    signed: true,
};

a._signed_matched_sign_extension(&mut b);

let actual_string = format!("{}", a);