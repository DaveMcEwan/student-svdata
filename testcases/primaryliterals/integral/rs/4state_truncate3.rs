let mut a = SvPrimaryLiteralIntegral {
    data_01: vec![1, 0],
    data_xz: Some(vec![0, 0]),
    size: 128,
    signed: false,
};

a._truncate(1);

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![1],
    data_xz: Some(vec![0]),
    size: 1,
    signed: false,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);