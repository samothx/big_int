use std::ops::{AddAssign, Add, SubAssign, Sub, MulAssign, Mul, DivAssign, Div};
use std::cmp::Ordering;

use super::BigInt;

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
        BigInt{
            signed: self.signed
        }
    }
}

impl MulAssign for BigInt {
    fn mul_assign(&mut self, other: Self) {
        todo!()
    }
}

impl Div for BigInt {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        todo!()
    }
}

impl DivAssign for BigInt {
    fn div_assign(&mut self, other: Self) {
        todo!()
    }
}
