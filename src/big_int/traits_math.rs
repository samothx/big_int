use std::ops::{AddAssign, Add, SubAssign, Sub, MulAssign, Mul, DivAssign, Div};
use super::BigInt;

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
        self.add_into(&other)
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
        self.sub_into(&other);
    }
}

impl Mul for BigInt {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self.mul_with(&other)
    }
}

impl MulAssign for BigInt {
    fn mul_assign(&mut self, other: Self) {
        self.mul_into(&other)
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
        self.uint = self.uint.div_mod_into(&other.uint);
    }
}
