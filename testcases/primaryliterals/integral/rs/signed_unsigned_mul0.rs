let a = SvPrimaryLiteralIntegral {
    data01: vec![3],
    size: 2,
    signed: false,
};

let b = SvPrimaryLiteralIntegral {
    data01: vec![4],
    size: 3,
    signed: true,
};

let c: SvPrimaryLiteralIntegral = a.mul(b.clone());

let exp = SvPrimaryLiteralIntegral {
    data01: vec![12],
    size: 4,
    signed: false,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);