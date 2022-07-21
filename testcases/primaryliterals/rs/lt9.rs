let a = SvPrimaryLiteral {
    data01: vec![0, 9223372036854775808],
    size: 65,
    signed: true,
};

let b = SvPrimaryLiteral {
    data01: vec![0, 9223372036854775808],
    size: 66,
    signed: true,
};

let c: bool = a < b;

let actual_string = format!("{}", c);