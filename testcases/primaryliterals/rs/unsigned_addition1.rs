let a = SvPrimaryLiteral {
    data01: vec![9223372036854775808],
    num_bits: 64,
    signed: false,
};

let b = SvPrimaryLiteral {
    data01: vec![9223372036854775808],
    num_bits: 64,
    signed: false,
};

let c: SvPrimaryLiteral = a.add_primlit(b.clone());

let exp = SvPrimaryLiteral {
    data01: vec![1, 0],
    num_bits: 65,
    signed: false,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);