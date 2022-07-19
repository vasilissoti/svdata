let a = SvPrimaryLiteral {
    data01: vec![0, 9223372036854775808],
    num_bits: 65,
    signed: true,
};

let b: SvPrimaryLiteral = a.lsl(1);

let exp = SvPrimaryLiteral {
    data01: vec![1, 0],
    num_bits: 66,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);