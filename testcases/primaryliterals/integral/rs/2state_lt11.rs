let a = SvPrimaryLiteralIntegral {
    data_01: vec![0, 9223372036854775808],
    data_xz: None,
    size: 65,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808],
    data_xz: None,
    size: 66,
    signed: false,
};

let c = a.lt(b.clone());

assert_eq!(c, logic1b_0());

let actual_string = format!("{}", c);