let a = SvPrimaryLiteral {
    data01: vec![9223372036854775809, 9223372036854775809],
    num_bits: 128,
    signed: true,
};

let b: SvPrimaryLiteral = a.lsr(2);

let actual_string = format!("{}", b);