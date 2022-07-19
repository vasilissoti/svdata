let a = SvPrimaryLiteral {
    data01: vec![9223372036854775808, 9223372036854775808, 1],
    num_bits: 192,
    signed: false,
};

let b = SvPrimaryLiteral {
    data01: vec![16],
    num_bits: 5,
    signed: false,
};

let c: SvPrimaryLiteral = a.mul(b.clone());

let exp = SvPrimaryLiteral {
    data01: vec![8, 8, 0, 16],
    num_bits: 196,
    signed: false,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);