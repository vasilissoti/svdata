let a = SvPrimaryLiteral {
    data01: vec![0, 9223372036854775809],
    num_bits: 65,
    signed: true,
};

let b = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    num_bits: 63,
    signed: true,
};

let c: SvPrimaryLiteral = a.cat(b.clone());

let exp = SvPrimaryLiteral {
    data01: vec![4611686018427387904, 13835058055282163712],
    num_bits: 128,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);