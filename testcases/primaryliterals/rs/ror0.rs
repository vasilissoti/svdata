let a = SvPrimaryLiteral {
    data01: vec![3, 9223372036854775809],
    num_bits: 65,
    signed: true,
};

let b: SvPrimaryLiteral = a.ror(2);

let actual_string = format!("{}", b);