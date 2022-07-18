let a = SvPrimaryLiteral {
    data01: vec![0, 9223372036854775808],
    num_bits: 65,
    signed: true,
};

let b: SvPrimaryLiteral = a.neg();

let actual_string = format!("{}", b);