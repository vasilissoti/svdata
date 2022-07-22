let a = SvPrimaryLiteral {
    data01: vec![9223372036854775809, 9223372036854775809],
    num_bits: 128,
    signed: true,
};

let b: SvPrimaryLiteral = a.ror(2);

let exp = SvPrimaryLiteral {
    data01: vec![6917529027641081856, 6917529027641081856],
    num_bits: 128,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);