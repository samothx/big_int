use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Debug};
use std::convert::TryFrom;

#[cfg(feature = "big_int")]
use crate::BigInt;

use super::BigUInt;

impl Default for BigUInt {
    fn default() -> Self {
        Self::new()
    }
}

impl Eq for BigUInt {}

impl PartialOrd for BigUInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigUInt {
    fn cmp(&self, other: &Self) -> Ordering {
        // eprintln!("cmp 0x{} length: {}, 0x{} length {}", self.to_hex_string(),self.length(), other.to_hex_string(), other.length());
        match self.length.cmp(&other.length) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                for (block1, block2) in self.bits.iter().rev().zip(other.bits.iter().rev()) {
                    match block1.cmp(block2) {
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => ()
                    }
                }
                Ordering::Equal
            }
        }
    }
}




impl From<u8> for BigUInt {
    fn from(src: u8) -> Self {
        BigUInt::from_u8(src)
    }
}

impl From<u16> for BigUInt {
    fn from(src: u16) -> Self {
        BigUInt::from_u16(src)
    }
}

impl From<u32> for BigUInt {
    fn from(src: u32) -> Self {
        BigUInt::from_u32(src)
    }
}

impl From<u64> for BigUInt {
    fn from(src: u64) -> Self {
        BigUInt::from_u64(src)
    }
}

impl From<u128> for BigUInt {
    fn from(src: u128) -> Self {
        BigUInt::from_u128(src)
    }
}

/// Requires feature big_int to be enabled
#[cfg(feature = "big_int")]
impl TryFrom<BigInt> for BigUInt {
    type Error = &'static str;
    fn try_from(value: BigInt) -> Result<Self, Self::Error> {
        if value.is_negative() {
            Err("BigUInt only accepts values >= 0")
        } else {
            Ok(value.as_unsigned())
        }
    }
}

impl TryFrom<BigUInt> for u64 {
    type Error = &'static str;
    fn try_from(value: BigUInt) -> Result<Self, Self::Error> {
        if let Some(res) = value.to_u64() {
            Ok(res)
        } else {
            Err("BigUInt is too big for u64")
        }
    }
}

impl TryFrom<BigUInt> for u128 {
    type Error = &'static str;
    fn try_from(value: BigUInt) -> Result<Self, Self::Error> {
        if let Some(res) = value.to_u128() {
            Ok(res)
        } else {
            Err("BigUInt is too big for u128")
        }
    }
}


impl Debug for BigUInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(L:{},0x{})", self.length, self.to_hex_string())
    }
}

impl Display for BigUInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_dec_string().as_str())
    }
}
