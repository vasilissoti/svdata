let mut a = SvPrimaryLiteral {
    data01: vec![0, 9223372036854775808],
    num_bits: 65,
    signed: true,
};

a._signed_sign_inversion();

let actual_string = format!("{}", a);