use std::ops::{Shl, ShlAssign, BitAnd, BitAndAssign, BitOrAssign, BitOr, AddAssign, Add, SubAssign, Sub, MulAssign, Mul, DivAssign, Div, Shr, ShrAssign};
use std::cmp::Ordering;

use super::BigUInt;
use std::fmt::{Display, Formatter, Debug};
use crate::BigInt;
use std::convert::TryFrom;

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

// TODO: create alternatives to all std::ops traits as they are often inefficient due to
//       BigUInt not implementing the copy trait
//       See SubAssign / BigUInt::sub_from()

impl Add for BigUInt {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.add_to(&other)
    }
}

impl AddAssign for BigUInt {
    fn add_assign(&mut self, other: Self) {
        self.add_to_self(&other);
    }
}

impl Sub for BigUInt {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.sub_from(&other)
    }
}

impl SubAssign for BigUInt {
    fn sub_assign(&mut self, other: Self) {
        self.sub_from_self(&other);
    }
}

impl Mul for BigUInt {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self.mul_with(&other)
    }
}

impl MulAssign for BigUInt {
    fn mul_assign(&mut self, other: Self) {
        self.mul_with_self(&other)
    }
}

impl Div for BigUInt {
    type Output = Self;
    fn div(self, other: Self) -> BigUInt {
        let (res, _) = self.div_mod(&other);
        res
    }
}

impl DivAssign for BigUInt {
    fn div_assign(&mut self, other: Self) {
        let _ = self.div_mod_self(&other);
    }
}


impl Shl<usize> for BigUInt {
    type Output = BigUInt;

    fn shl(self, rhs: usize) -> Self::Output {
        if rhs == 0 {
            self
        } else {
            self.left_shift(rhs)
        }
    }
}

impl ShlAssign<usize> for BigUInt {
    fn shl_assign(&mut self, rhs: usize) {
        self.left_shift_self(rhs)
    }
}



impl Shr<usize> for BigUInt {
    type Output = BigUInt;

    fn shr(self, rhs: usize) -> Self::Output {
        if rhs == 0 {
            self
        } else {
            self.right_shift(rhs)
        }
    }
}

impl ShrAssign<usize> for BigUInt {
    fn shr_assign(&mut self, rhs: usize) {
        self.right_shift_self(rhs)
    }
}

impl BitAnd for BigUInt {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a & b`
    fn bitand(self, rhs: Self) -> Self {
        let mut bits = vec![];
        for (block1, block2) in self.bits.iter().zip(rhs.bits.iter()) {
            bits.push(*block1 & *block2);
        }


        let mut res = BigUInt {
            length: usize::max(self.length, rhs.length),
            bits,
        };
        res.trim();
        res
    }
}

impl BitAndAssign for BigUInt {
    // rhs is the "right-hand side" of the expression `a &= b`
    fn bitand_assign(&mut self, rhs: Self) {
        for (block1, block2) in self.bits.iter_mut().zip(rhs.bits.iter()) {
            *block1 &= *block2;
        }

        self.trim();
    }
}

impl BitOr for BigUInt {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a &= b`
    fn bitor(self, rhs: Self) -> Self {
        let mut bits = vec![];
        for (block1, block2) in self.bits.iter().zip(rhs.bits.iter()) {
            bits.push(*block1 | *block2);
        }

        if self.bits.len() < rhs.bits.len() {
            bits.extend_from_slice(&rhs.bits[self.bits.len()..]);
        }

        let mut res = BigUInt {
            length: usize::max(self.length, rhs.length),
            bits,
        };

        res.trim();
        res
    }
}

impl BitOrAssign for BigUInt {
    // rhs is the "right-hand side" of the expression `a &= b`
    fn bitor_assign(&mut self, rhs: Self) {
        for (block1, block2) in self.bits.iter_mut().zip(rhs.bits.iter()) {
            *block1 |= *block2;
        }

        if self.bits.len() < rhs.bits.len() {
            self.bits.extend_from_slice(&rhs.bits[self.bits.len()..]);
        }
        self.length = usize::max(self.length, rhs.length);

        self.trim();
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
