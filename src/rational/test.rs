use super::Rational;



#[test]
fn test_rat_from() {
    let rat = Rational::from((1,3));
    assert_eq!(format!("{}",rat).as_str(),"1/3");
    assert_eq!(format!("{:?}",rat).as_str(),"((L:1,0x1)/(L:2,0x3))");
}
#[test]
fn test_rat_from_f64() {
    let max_error = 1.0 / 2.0f64.powi(f64::MANTISSA_DIGITS as i32);
    eprintln!("max error: {:e}", max_error);
    let test_val = vec![1.9f64, 1.0/3.0, 2.0_f64.sqrt(), 3.0_f64.sqrt(), std::f64::consts::PI];
    for src in test_val {
        let rat = Rational::from_f64(src);
        let error = rat.to_f64() - src;
        // eprintln!("src: {}, rat: {}, error: {:e}", src, rat, error);
        assert!(error.abs() < max_error);
    }
}

#[test]
fn test_rat_sqrt() {
    let max_error: Rational = (1.0 / 2.0f64.powi(f64::MANTISSA_DIGITS as i32)).into();
    eprintln!("max error: {}", max_error);
    let test_val: Vec<Rational> = vec![2u32.into(), 3u32.into(), 4u32.into(), 5u32.into() ];
    for src in test_val {
        let sqrt = src.sqrt();
        let pow2 = sqrt.powi(2);
        let error = sqrt.sub_from(&sqrt).abs();
        assert!(error.abs() < max_error);
    }

}
