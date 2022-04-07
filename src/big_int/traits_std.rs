use std::fmt::{Debug, Formatter, Display};
use std::convert::TryFrom;

use super::BigInt;
use crate::BigUInt;
use std::cmp::Ordering;


impl Eq for BigInt {}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.signed {
            if other.signed {
                // both signed - invert result
                match self.uint.cmp(&other.uint) {
                    Ordering::Greater => Ordering::Less,
                    Ordering::Less => Ordering::Greater,
                    Ordering::Equal => Ordering::Equal
                }
            } else {
                Ordering::Less
            }
        } else if other.signed {
            Ordering::Greater
        } else {
            // both unsigned - compare
            self.uint.cmp(&other.uint)
        }
    }
}

impl From<i8> for BigInt {
    fn from(src: i8) -> Self {
        BigInt::from_i8(src)
    }
}

impl From<i16> for BigInt {
    fn from(src: i16) -> Self {
        BigInt::from_i16(src)
    }
}

impl From<i32> for BigInt {
    fn from(src: i32) -> Self {
        BigInt::from_i32(src)
    }
}

impl From<i64> for BigInt {
    fn from(src: i64) -> Self {
        BigInt::from_i64(src)
    }
}

impl From<i128> for BigInt {
    fn from(src: i128) -> Self {
        BigInt::from_i128(src)
    }
}

impl From<BigUInt> for BigInt {
    fn from(value: BigUInt) -> Self {
        Self{
            signed: false,
            uint: value
        }
    }
}

impl TryFrom<BigInt> for i64 {
    type Error = &'static str;
    fn try_from(value: BigInt) -> Result<Self, Self::Error> {
        if let Some(res) = value.to_i64() {
            Ok(res)
        } else {
            Err("BigInt is too big for i64")
        }
    }
}

impl TryFrom<BigInt> for i128 {
    type Error = &'static str;
    fn try_from(value: BigInt) -> Result<Self, Self::Error> {
        if let Some(res) = value.to_i128() {
            Ok(res)
        } else {
            Err("BigInt is too big for i128")
        }
    }
}


impl Debug for BigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(L:{},{})", self.uint.length(), self)
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_dec_str().as_str())
    }
}
