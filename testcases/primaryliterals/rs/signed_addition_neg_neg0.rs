let mut a = SvPrimaryLiteral {
    data01: vec![9223372036854775808],
    num_bits: 64,
    signed: true,
};

a.add_usize(9223372036854775808);

let actual_string = format!("{}", a);