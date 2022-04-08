use std::cmp::{Ordering};
use rand::Rng;
use super::{BigUInt, BIT_64};

#[test]
fn test_from_u8() {
    let bf = BigUInt::from_u8(0);
    assert!(bf.is_zero());
    let bf = BigUInt::from_u8(1);
    assert!(!bf.is_zero());
    assert_eq!(bf.length(), 1);
    assert_eq!(bf.to_bin_string(), "1");
    let bf = BigUInt::from_u8(0x80);
    assert!(!bf.is_zero());
    assert_eq!(bf.length(), 8);
    assert_eq!(bf.to_bin_string(), "10000000");
    assert_eq!(bf.to_hex_string(), "80");
}

#[test]
fn test_from_u16() {
    let bf = BigUInt::from_u16(0);
    assert!(bf.is_zero());
    let bf = BigUInt::from_u16(1);
    assert!(!bf.is_zero());
    assert_eq!(bf.length(), 1);
    assert_eq!(bf.to_bin_string(), "1");
    let bf = BigUInt::from_u16(0x8000);
    assert!(!bf.is_zero());
    assert_eq!(bf.length(), 16);
    assert_eq!(bf.to_bin_string(), "1000000000000000");
}

#[test]
fn test_from_u32() {
    let bf = BigUInt::from_u32(0);
    assert!(bf.is_zero());
    let bf = BigUInt::from_u32(1);
    assert!(!bf.is_zero());
    assert_eq!(bf.length(), 1);
    assert_eq!(bf.to_bin_string(), "1");
    let bf = BigUInt::from_u32(0x80000000);
    assert!(!bf.is_zero());
    assert_eq!(bf.length(), 32);
    assert_eq!(bf.to_hex_string(), "80000000");
}

#[test]
fn test_from_u64() {
    let bf = BigUInt::from_u64(0);
    assert!(bf.is_zero());
    let bf = BigUInt::from_u64(1);
    assert!(!bf.is_zero());
    assert_eq!(bf.length(), 1);
    assert_eq!(bf.to_bin_string(), "1");
    let bf = BigUInt::from_u64(0x8000000000000000);
    assert!(!bf.is_zero());
    assert_eq!(bf.length(), 64);
    assert_eq!(bf.to_hex_string(), "8000000000000000");
}

#[test]
fn test_from_u128() {
    let bf = BigUInt::from_u128(0);
    assert!(bf.is_zero());
    let bf = BigUInt::from_u128(1);
    assert!(!bf.is_zero());
    assert_eq!(bf.length(), 1);
    assert_eq!(bf.to_bin_string(), "1");
    let bf = BigUInt::from_u128(0x80000000000000000000000000000000);
    assert!(!bf.is_zero());
    assert_eq!(bf.length(), 128);
    assert_eq!(bf.to_hex_string(), "80000000000000000000000000000000");
}

#[test]
fn test_to_hex_string() {
    let bf = BigUInt::from_u16(0b10000000);
    assert_eq!(bf.to_hex_string(), "80");
    let bf = BigUInt::from_u16(0b1000000);
    assert_eq!(bf.to_hex_string(), "40");
    let bf = BigUInt::from_u16(0b100000);
    assert_eq!(bf.to_hex_string(), "20");
    let bf = BigUInt::from_u16(0b10000);
    assert_eq!(bf.to_hex_string(), "10");
    let bf = BigUInt::from_u16(0b1000);
    assert_eq!(bf.to_hex_string(), "8");
    let bf = BigUInt::from_u32(0x39CE739C);
    assert_eq!(bf.to_hex_string(), "39CE739C");
}

#[test]
fn test_left_shift() {
    let bf = BigUInt::from_u64(0x1);
    let res = bf << 1;
    assert_eq!(res.to_hex_string(), "2");
    let bf = BigUInt::from_u64(0x39CE739C);
    let res = bf << 1;
    assert_eq!(res.to_hex_string(), "739CE738");
    let bf = BigUInt::from_u64(0x739CE739CE739CE7);
    let res = bf << 1;
    assert_eq!(res.to_hex_string(), "E739CE739CE739CE");

    let bf = BigUInt::from_u64(0x739CE739CE739CE7);
    let res = bf << 2;
    assert_eq!(res.to_bin_string(), format!("{:b}", 0x739CE739CE739CE7u128 << 2));

    let bf = BigUInt::from_u64(0x739CE739CE739CE7);
    let res = bf << 64;
    assert_eq!(res.to_bin_string(), format!("{:b}", 0x739CE739CE739CE7u128 << 64));
}

#[test]
fn test_left_shift_assign() {
    let mut bf = BigUInt::from_u64(0x1);
    bf <<= 1;
    assert_eq!(bf.to_hex_string(), "2");
    let mut bf = BigUInt::from_u64(0x39CE739C);
    bf <<= 1;
    assert_eq!(bf.to_hex_string(), "739CE738");
    let mut bf = BigUInt::from_u64(0x739CE739CE739CE7);
    bf <<= 1;
    assert_eq!(bf.to_hex_string(), "E739CE739CE739CE");

    let mut bf = BigUInt::from_u64(0x739CE739CE739CE7);
    bf <<= 2;
    assert_eq!(bf.to_bin_string(), format!("{:b}", 0x739CE739CE739CE7u128 << 2));

    let mut bf = BigUInt::from_u64(0x739CE739CE739CE7);
    bf <<= 64;
    assert_eq!(bf.to_bin_string(), format!("{:b}", 0x739CE739CE739CE7u128 << 64));
}


#[test]
fn test_right_shift() {
    let bi: BigUInt = 0x1u32.into();
    assert!((bi >> 1).is_zero());

    let bi: BigUInt = 0x1u32.into();
    assert_eq!(bi >> 0, 0x1u32.into());

    let bi: BigUInt = 0xF0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0u128.into();
    assert_eq!(bi >> 3, 0x1E1E1E1E1E1E1E1E1E1E1E1E1E1E1E1Eu128.into());

    let bi: BigUInt = 0xF0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0u128.into();
    assert_eq!(bi >> 64, 0xF0F0F0F0F0F0F0F0u128.into());

    let bi: BigUInt = 0xE7E7E7E7E7E7E7E7E7E7E7E7E7E7E7E7u128.into();
    assert_eq!(bi >> 3, 0x1CFCFCFCFCFCFCFCFCFCFCFCFCFCFCFCu128.into());

    let bi: BigUInt = 0xF0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0u128.into();
    assert_eq!(bi >> 67, 0x1E1E1E1E1E1E1E1Eu128.into());
}

#[test]
fn test_right_shift_into() {
    let mut bi: BigUInt = 0x1u32.into();
    bi >>= 1;
    assert!(bi.is_zero());

    let mut bi: BigUInt = 0x1u32.into();
    bi >>= 0;
    assert_eq!(bi, 0x1u32.into());

    let mut bi: BigUInt = 0xF0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0u128.into();
    bi >>= 3;
    assert_eq!(bi, 0x1E1E1E1E1E1E1E1E1E1E1E1E1E1E1E1Eu128.into());

    let mut bi: BigUInt = 0xF0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0u128.into();
    bi >>= 64;
    assert_eq!(bi, 0xF0F0F0F0F0F0F0F0u128.into());

    let mut bi: BigUInt = 0xE7E7E7E7E7E7E7E7E7E7E7E7E7E7E7E7u128.into();
    bi >>= 3;
    assert_eq!(bi, 0x1CFCFCFCFCFCFCFCFCFCFCFCFCFCFCFCu128.into());

    let mut bi: BigUInt = 0xF0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0u128.into();
    bi >>= 67;
    assert_eq!(bi, 0x1E1E1E1E1E1E1E1Eu128.into());
}

#[test]
fn test_and() {
    let bf1 = BigUInt::from_u128(0xF0F0F0F0F0F0F0F0);
    let bf2 = BigUInt::from_u128(0x3C3C3C3C3C3C3C3C);
    let res = bf1 & bf2;
    assert_eq!(res.to_bin_string(), format!("{:b}", 0xF0F0F0F0F0F0F0F0u128 & 0x3C3C3C3C3C3C3C3Cu128));

    let mut bf1 = BigUInt::from_u128(0xF0F0F0F0F0F0F0F0);
    let bf2 = BigUInt::from_u128(0x3C3C3C3C3C3C3C3C);
    bf1 &= bf2;
    assert_eq!(bf1.to_bin_string(), format!("{:b}", 0xF0F0F0F0F0F0F0F0u128 & 0x3C3C3C3C3C3C3C3Cu128));
}

#[test]
fn test_or() {
    let bf1 = BigUInt::from_u128(0xF0F0F0F0F0F0F0F0);
    let bf2 = BigUInt::from_u128(0x3C3C3C3C3C3C3C3C);
    let res = bf1 | bf2;
    assert_eq!(res.to_bin_string(), format!("{:b}", 0xF0F0F0F0F0F0F0F0u128 | 0x3C3C3C3C3C3C3C3Cu128));

    let mut bf1 = BigUInt::from_u128(0xF0F0F0F0F0F0F0F0);
    let bf2 = BigUInt::from_u128(0x3C3C3C3C3C3C3C3C);
    bf1 |= bf2;
    assert_eq!(bf1.to_bin_string(), format!("{:b}", 0xF0F0F0F0F0F0F0F0u128 | 0x3C3C3C3C3C3C3C3Cu128));
}


#[test]
fn test_gcd() {
    let bi1: BigUInt = 5u32.into();
    let bi2: BigUInt = 7u32.into();
    assert_eq!(bi1.gcd(&bi2), 1u32.into());

    let bi1: BigUInt = 5u32.into();
    let bi2: BigUInt = 25u32.into();
    assert_eq!(bi1.gcd(&bi2), 5u32.into());

    let mut rng = rand::thread_rng();

    let gcd: BigUInt = rng.gen_range(0xFFFFFFFF00000000u64..=0xFFFFFFFFFFFFFFFFu64).into();
    // eprintln!("gcd: {:?}, {}", gcd, gcd);

    let nums: Vec<BigUInt> = (0..4).map(|_| {
        rng.gen_range(0..0x80000000u64).into()
    }).collect();
    let mut bi1 = gcd.clone();
    (1..5u32).zip(nums.iter()).for_each(|(offset, num)| { bi1 *= num.add_to(&offset.into()); });
    // eprintln!("bi1: {:?}, {}", bi1, bi1);
    let (_, modulo) = bi1.div_mod(&gcd);
    assert!(modulo.is_zero());
    let mut bi2 = gcd.clone();
    (5..10u32).zip(nums.iter()).for_each(|(offset, num)| { bi2 *= num.add_to(&offset.into()); });
    let (_, modulo) = bi2.div_mod(&gcd);
    assert!(modulo.is_zero());
    let wrong_gcd = gcd.add_to(&1u32.into());
    let (_, modulo) = bi2.div_mod(&wrong_gcd);
    assert!(!modulo.is_zero());
    // eprintln!("bi2: {:?}, {}", bi2, bi2);
    let calc_gcd = bi1.gcd(&bi2);
    let (_, modulo) = bi1.div_mod(&calc_gcd);
    assert!(modulo.is_zero());
    let (_, modulo) = bi2.div_mod(&calc_gcd);
    assert!(modulo.is_zero());
    if gcd != calc_gcd {
        assert!(gcd < calc_gcd);
    }
}

#[test]
fn test_add() {
    let mut bf1 = BigUInt::from_u128(0x1);
    let bf2 = BigUInt::from_u128(0x1);
    bf1 += bf2;
    assert_eq!(bf1.length(), 2);
    assert_eq!(bf1.to_hex_string(), "2");

    let mut bf1 = BigUInt::from_u64(0xFFFFFFFFFFFFFFFF);
    let bf2 = BigUInt::from_u64(0xFFFFFFFFFFFFFFFF);
    bf1 += bf2;
    //assert_eq!(bf1.length(), 2);
    assert_eq!(bf1.to_hex_string(), format!("{:X}", 0xFFFFFFFFFFFFFFFFu128 + 0xFFFFFFFFFFFFFFFFu128));
}

#[test]
fn test_sub() {
    let mut bi = BigUInt::from_u128(0x8AC7230489E8000000);
    bi -= BigUInt::from_u32(0x5DEAD34);
    assert_eq!(bi.to_hex_string(), format!("{:X}", 0x8AC7230489E8000000u128 - 0x5DEAD34u128));

    let bi = BigUInt::from_u128(0x8AC7230489E8000000);
    let res = bi - BigUInt::from_u32(0x5DEAD34);
    assert_eq!(res.to_hex_string(), format!("{:X}", 0x8AC7230489E8000000u128 - 0x5DEAD34u128));
}

#[test]
#[should_panic]
fn test_sub_fail() {
    let mut bi = BigUInt::from_u32(10000);
    bi -= BigUInt::from_u32(50000);
}

#[test]
fn test_mul() {
    let mut bi1 = BigUInt::from_u64(BIT_64);
    let b12 = BigUInt::from_u32(0xDEAD);
    let mut check = BIT_64 as u128;
    for _ in 0..4 {
        bi1 *= b12.clone();
        check *= 0xDEAD;
        assert_eq!(bi1.to_hex_string(), format!("{:X}", check));
    }

    let mut bi1 = BigUInt::from_u64(BIT_64);
    let b12 = BigUInt::from_u32(0xDEAD);
    let mut check = BIT_64 as u128;
    for _ in 0..4 {
        bi1 = bi1 * b12.clone();
        check *= 0xDEAD;
        assert_eq!(bi1.to_hex_string(), format!("{:X}", check));
    }

    let bi1: BigUInt = 0x7FFFFFFFFFFFFFFFu64.into();
    let bi2: BigUInt = 0x4u8.into();
    assert_eq!((bi1 * bi2).to_u128(), Some(0x1FFFFFFFFFFFFFFFC));
}

#[test]
fn test_shift_out() {
    let mut bi = BigUInt::from_u64(0x823456789ABCDEF0);
    let res = bi.shift_out(16);
    assert_eq!(bi.to_hex_string(), "56789ABCDEF0");
    assert_eq!(res.to_hex_string(), "8234");

    let mut bi = BigUInt::from_u64(0x123456789ABCDEF0);
    let res = bi.shift_out(13);
    assert_eq!(bi.to_hex_string(), "56789ABCDEF0");
    assert_eq!(res.to_hex_string(), "1234");

    let mut bi = BigUInt::from_u128(0x123456789ABCDEF01234);
    let res = bi.shift_out(13);
    assert_eq!(bi.to_hex_string(), "56789ABCDEF01234");
    assert_eq!(res.to_hex_string(), "1234");

    let test = 0x123456789ABCDEF01234;
    let mut bi = BigUInt::from_u128(test);
    let res = bi.shift_out(16);
    let expected_res = ((0xFFFFu128 << 61) & test) >> 61;
    assert_eq!(bi.to_hex_string(), "16789ABCDEF01234");
    assert_eq!(res.to_hex_string(), format!("{:X}", expected_res));
}

#[test]
fn test_get_bits() {
    let bi = BigUInt::from_u128(0xF0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0);
    let res = bi.get_bits(127, 64);
    assert_eq!(res.to_hex_string(), "F0F0F0F0F0F0F0F0");

    let res = bi.get_bits(63, 64);
    assert_eq!(res.to_hex_string(), "F0F0F0F0F0F0F0F0");

    let res = bi.get_bits(95, 64);
    assert_eq!(res.to_hex_string(), "F0F0F0F0F0F0F0F0");

    let res = bi.get_bits(94, 64);
    assert_eq!(res.to_hex_string(), "E1E1E1E1E1E1E1E1");
}

#[test]
fn test_set() {
    let mut bi = BigUInt::new();
    assert_eq!(bi.set(7, true), None);
    assert_eq!(bi.to_hex_string(), "80");
    assert_eq!(bi.set(6, true), Some(false));
    assert_eq!(bi.to_hex_string(), "C0");
    assert_eq!(bi.set(7, false), Some(true));
    assert_eq!(bi.to_hex_string(), "40");
    assert_eq!(bi.set(6, false), Some(true));
    assert_eq!(bi.to_hex_string(), "0");
}

#[test]
fn test_from_hex_str() {
    let bi = BigUInt::from_hex_str("1234567890")
        .expect("failed to convert from hex");
    assert_eq!(bi.to_hex_string(), "1234567890");

    let bi = BigUInt::from_hex_str("1234567890ABCDEF")
        .expect("failed to convert from hex");
    assert_eq!(bi.to_hex_string(), "1234567890ABCDEF");

    let bi = BigUInt::from_hex_str("1234567890ABCDEF1")
        .expect("failed to convert from hex");
    assert_eq!(bi.to_hex_string(), "1234567890ABCDEF1");

    let bi = BigUInt::from_hex_str("113572E4620B646BD672F2DEDCF983AC855B8ABAD93F")
        .expect("failed to convert from hex");
    assert_eq!(bi.to_hex_string(), "113572E4620B646BD672F2DEDCF983AC855B8ABAD93F");

    let bi = BigUInt::from_hex_str("169626BF76566EDF05BAFBCE9A13390D3F79FB6BD673")
        .expect("failed to convert from hex");
    assert_eq!(bi.to_hex_string(), "169626BF76566EDF05BAFBCE9A13390D3F79FB6BD673");
}


#[test]
fn test_cmp() {
    let bi1 = BigUInt::from_u32(0x2000);
    let bi2 = BigUInt::from_u32(0x2000);
    assert_eq!(bi1.cmp(&bi2), Ordering::Equal);
    let bi2 = BigUInt::from_u32(0x3000);
    assert_eq!(bi1.cmp(&bi2), Ordering::Less);
    let bi2 = BigUInt::from_u32(0x1000);
    assert_eq!(bi1.cmp(&bi2), Ordering::Greater);

    let bi1 = BigUInt::from_hex_str("113572E4620B646BD672F2DEDCF983AC855B8ABAD93F");
    let bi2 = BigUInt::from_hex_str("169626BF76566EDF05BAFBCE9A13390D3F79FB6BD673");
    assert!(bi1 < bi2);
}

#[test]
fn test_div() {
    let mut bi = BigUInt::from_u32(0x80000000);
    bi /= BigUInt::from_u32(0x3000);
    assert_eq!(bi.to_hex_string(), "2AAAA");

    let mut bi = BigUInt::from_u32(0x80000000);
    let modulo = bi.div_mod_into(&BigUInt::from_u32(0x3000));
    assert_eq!(bi.to_hex_string(), "2AAAA");
    assert_eq!(modulo.to_hex_string(), "2000");
}

#[test]
fn test_to_dec_string() {
    let bi = BigUInt::from_u64(0xAB54A98F81652440);
    assert_eq!(bi.to_dec_string(), "12345678912345678912");
}

#[test]
fn test_to_f64() {
    let max_error = 10.0 / 2.0f64.powi(f64::MANTISSA_DIGITS as i32);
    let mut fact_bi = BigUInt::from(1u32);
    let mut fact_f64 = 1.0_f64;
    for idx in 2..1000u32 {
        fact_bi.mul_into(&idx.into());
        fact_f64 *= f64::from(idx);
        if fact_f64.is_finite() {
            assert!(((fact_f64 - fact_bi.to_f64().expect("cannot convert to f64")) / fact_f64).abs() < max_error,
                    "error too big: err:{:e}>max_err:{:e}",
                    ((fact_f64 - fact_bi.to_f64().expect("cannot convert to f64")) / fact_f64).abs(), max_error);
        } else {
            break;
        }
    }
}

#[test]
fn test_from_f64() {
    let mut factorial = 1f64;
    let max_error = 10.0 / 2.0f64.powi(f64::MANTISSA_DIGITS as i32);
    for idx in 2..1000u32 {
        factorial *= f64::from(idx);

        if factorial.is_infinite() {
            break;
        } else {
            let f_int = BigUInt::from_f64(factorial);
            assert!(((f_int.to_f64().expect("cannot convert to f64") - factorial) / factorial).abs() < max_error,
                    "error too big: err:{:e}>max_err:{:e}",
                    ((f_int.to_f64().expect("cannot convert to f64") - factorial) / factorial).abs(),
                    max_error);
        }
    }
}
