let mut a = SvPrimaryLiteral {
    data01: vec![4611686018427387904],
    num_bits: 64,
    signed: true,
};

a._neg();

let actual_string = format!("{}", a);