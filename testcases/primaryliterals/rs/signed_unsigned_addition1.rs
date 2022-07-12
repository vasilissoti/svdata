let mut a = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    num_bits: 64,
    signed: true,
};

let b = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    num_bits: 63,
    signed: false,
};

a.add_primlit(b.clone());

let actual_string = format!("{}", a);
