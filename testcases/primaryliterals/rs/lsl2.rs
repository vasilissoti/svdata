let a = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    num_bits: 64,
    signed: true,
};

let b: SvPrimaryLiteral = a.lsl(4);

let actual_string = format!("{}", b);