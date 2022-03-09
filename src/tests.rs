
use super::*;

#[test]
fn test_from_u8() {
    let bf = BigUInt::from_u8(0);
    assert!(bf.is_empty());
    let bf = BigUInt::from_u8(1);
    assert!(!bf.is_empty());
    assert_eq!(bf.length(), 1);
    assert_eq!(bf.to_bin_string(), "1");
    let bf = BigUInt::from_u8(0x80);
    assert!(!bf.is_empty());
    assert_eq!(bf.length(), 8);
    assert_eq!(bf.to_bin_string(), "10000000");
    assert_eq!(bf.to_hex_string(), "80");
}

#[test]
fn test_from_u16() {
    let bf = BigUInt::from_u16(0);
    assert!(bf.is_empty());
    let bf = BigUInt::from_u16(1);
    assert!(!bf.is_empty());
    assert_eq!(bf.length(), 1);
    assert_eq!(bf.to_bin_string(), "1");
    let bf = BigUInt::from_u16(0x8000);
    assert!(!bf.is_empty());
    assert_eq!(bf.length(), 16);
    assert_eq!(bf.to_bin_string(), "1000000000000000");
}

#[test]
fn test_from_u32() {
    let bf = BigUInt::from_u32(0);
    assert!(bf.is_empty());
    let bf = BigUInt::from_u32(1);
    assert!(!bf.is_empty());
    assert_eq!(bf.length(), 1);
    assert_eq!(bf.to_bin_string(), "1");
    let bf = BigUInt::from_u32(0x80000000);
    assert!(!bf.is_empty());
    assert_eq!(bf.length(), 32);
    assert_eq!(bf.to_hex_string(), "80000000");
}

#[test]
fn test_from_u64() {
    let bf = BigUInt::from_u64(0);
    assert!(bf.is_empty());
    let bf = BigUInt::from_u64(1);
    assert!(!bf.is_empty());
    assert_eq!(bf.length(), 1);
    assert_eq!(bf.to_bin_string(), "1");
    let bf = BigUInt::from_u64(0x8000000000000000);
    assert!(!bf.is_empty());
    assert_eq!(bf.length(), 64);
    assert_eq!(bf.to_hex_string(), "8000000000000000");
}

#[test]
fn test_from_u128() {
    let bf = BigUInt::from_u128(0);
    assert!(bf.is_empty());
    let bf = BigUInt::from_u128(1);
    assert!(!bf.is_empty());
    assert_eq!(bf.length(), 1);
    assert_eq!(bf.to_bin_string(), "1");
    let bf = BigUInt::from_u128(0x80000000000000000000000000000000);
    assert!(!bf.is_empty());
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
fn test_shift() {
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
fn test_shift_assign() {
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

/*
#[test]
fn test_to_sparse() {
    let mut bf1 = BigInt::from_u128(0x1);
    assert_eq!(bf1.length(), 128);
    bf1.to_sparse();
    assert_eq!(bf1.length(), 1);
}
*/

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
