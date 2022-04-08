use super::Rational;
use crate::BigUInt;

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
        let error = rat.to_f64().expect("cannot convert to f64") - src;
        // eprintln!("src: {}, rat: {}, error: {:e}", src, rat, error);
        assert!(error.abs() < max_error);
    }
}

#[test]
fn test_invert_rat() {
    let r1 =  Rational::from(2u32);
    let res = r1.invert();
    assert_eq!(res,(1u32,2u32).into());
}

#[test]
fn test_mul_rat() {
    let r1 =  Rational::from(-1i32);
    let r2: Rational = (1u32,2u32).into();

    let res = r1 * r2;
    assert_eq!(res,(-1i32,2i32).into());
}

#[test]
fn test_div_rat() {
    let r1 =  Rational::from(-1i32);
    let r2: Rational = 2u32.into();
    assert_eq!(r1/r2,(-1i32,2i32).into());
}

#[test]
fn test_rat_sqrt() {
    let max_error: Rational = (1.0 / 2.0f64.powi(f64::MANTISSA_DIGITS as i32)).into();
    eprintln!("max error: {}", max_error);
    let test_val: Vec<Rational> = vec![
        2u32.into(), 3u32.into(), 4u32.into(), 5u32.into(),100u32.into(), 10000u32.into(), 1000000u32.into(),
        BigUInt::from_hex_str("113572E4620B646BD672F2DEDCF983AC855B8ABAD93F")
            .expect("Failed to create BigUInt from hex string ").into()];
    for src in test_val {
        eprintln!("calc sqrt");
        let sqrt = src.sqrt();
        eprintln!("calc powi");
        let _pow2 = sqrt.powi(2);
        eprintln!("calc error");
        let error = sqrt.sub_from(&sqrt).abs();
        eprintln!("done");
        assert!(error.abs() < max_error);
    }
}

#[test]
fn test_rat_sub() {
    let rat1: Rational = 10u32.into();
    let rat2: Rational = (-3i32).into();
    assert_eq!(rat1 - rat2,13u32.into());
}
