let a = SvPrimaryLiteral {
    data01: vec![4611686018427387904, 4611686018427387904],
    num_bits: 127,
    signed: false,
};

let b: SvPrimaryLiteral = a.inv();

let actual_string = format!("{}", b);