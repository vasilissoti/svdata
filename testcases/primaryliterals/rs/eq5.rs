let a = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    size: 63,
    signed: true,
};

let b = SvPrimaryLiteral {
    data01: vec![9223372036854775808],
    size: 64,
    signed: true,
};

let c: bool = a == b;

let actual_string = format!("{}", c);