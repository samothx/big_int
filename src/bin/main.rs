
use simple_big_int::Rational;

fn main() {
    eprintln!("hello world");
    let max_error: Rational = (1.0 / 2.0f64.powi(f64::MANTISSA_DIGITS as i32)).into();
    eprintln!("max error: {}", max_error);
    let test_val: Vec<Rational> = vec![2u32.into(), 3u32.into(), 4u32.into(), 5u32.into() ];
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
