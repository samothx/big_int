
use simple_big_int::{Rational, BigUInt};

fn main() {
    eprintln!("hello world");
    let max_error: Rational = (1.0 / 2.0f64.powi(f64::MANTISSA_DIGITS as i32)).into();
    eprintln!("max error: {}", max_error);
    let test_val: Vec<Rational> = vec![
        2u32.into(), 3u32.into(), 4u32.into(), 5u32.into(),100u32.into(), 10000u32.into(), 1000000u32.into(),
        BigUInt::from_hex_str("113572E4620B646BD672F2DEDCF983AC855B8ABAD93F")
            .expect("Failed to create BigUInt from hex string ").into()];

    for src in test_val {
        // eprintln!("calc sqrt");
        let sqrt = src.sqrt();
        // eprintln!("calc powi");
        let pow2 = sqrt.powi(2);
        // eprintln!("calc error");
        let error = src.sub_from(&pow2).abs();
        assert!(error.abs() < max_error);
        eprintln!("done {}", src);
    }

}
