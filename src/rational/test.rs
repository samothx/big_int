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
