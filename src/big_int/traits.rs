use std::ops::{AddAssign, Add, SubAssign, Sub, MulAssign, Mul, DivAssign, Div};
use std::cmp::Ordering;

use super::BigInt;
use std::fmt::{Debug, Formatter, Display};
use crate::BigUInt;
use std::convert::TryFrom;

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

// TODO: create alternatives to all std::ops traits as they are often inefficient due to
//       BigUInt not implementing the copy trait
//       See SubAssign / BigUInt::sub_from()

impl Add for BigInt {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.add_to(&other)
    }
}

impl AddAssign for BigInt {
    fn add_assign(&mut self, other: Self) {
        self.add_to_self(&other)
    }
}

impl Sub for BigInt {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.sub_from(&other)
    }
}

impl SubAssign for BigInt {
    fn sub_assign(&mut self, other: Self) {
        self.sub_from_self(&other);
    }
}

impl Mul for BigInt {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        BigInt {
            signed: self.signed ^ other.signed,
            uint: self.uint * other.uint,
        }
    }
}

impl MulAssign for BigInt {
    fn mul_assign(&mut self, other: Self) {
        self.signed = self.signed ^ other.signed;
        self.uint.mul_assign(other.uint);
    }
}

impl Div for BigInt {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        BigInt {
            signed: self.signed ^ other.signed,
            uint: self.uint / other.uint,
        }
    }
}

impl DivAssign for BigInt {
    fn div_assign(&mut self, other: Self) {
        self.signed = self.signed ^ other.signed;
        self.uint = self.uint.div_mod_self(&other.uint);
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
