let mut a = SvPrimaryLiteral {
    data01: vec![4611686018427387904, 4611686018427387904],
    num_bits: 128,
    signed: true,
};

let b = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    num_bits: 64,
    signed: true,
};

a.prim_lit_add(b.clone());

let actual_string = format!("{}", a);