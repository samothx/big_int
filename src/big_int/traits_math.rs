use std::ops::{AddAssign, Add, SubAssign, Sub, MulAssign, Mul, DivAssign, Div};
use super::BigInt;

impl Add for BigInt {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.add_to(&other)
    }
}

impl<'a> Add<&'a Self> for BigInt {
    type Output = Self;

    fn add(self, other: &'a Self) -> Self::Output {
        self.add_to(other)
    }
}

impl<'a> Add<&'a BigInt> for &BigInt {
    type Output = BigInt;

    fn add(self, other: &'a BigInt) -> Self::Output {
        self.add_to(other)
    }
}

impl AddAssign for BigInt {
    fn add_assign(&mut self, other: Self) {
        self.add_into(&other)
    }
}

impl<'a> AddAssign<&'a Self> for BigInt {
    fn add_assign(&mut self, other: &'a Self) {
        self.add_into(other)
    }
}

impl Sub for BigInt {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.sub_from(&other)
    }
}

impl<'a> Sub<&'a Self> for BigInt {
    type Output = Self;

    fn sub(self, other: &'a Self) -> Self::Output {
        self.sub_from(other)
    }
}

impl<'a> Sub<&'a BigInt> for &BigInt {
    type Output = BigInt;

    fn sub(self, other: &'a BigInt) -> Self::Output {
        self.sub_from(other)
    }
}

impl SubAssign for BigInt {
    fn sub_assign(&mut self, other: Self) {
        self.sub_into(&other);
    }
}

impl<'a> SubAssign<&'a Self> for BigInt {
    fn sub_assign(&mut self, other: &'a Self) {
        self.sub_into(other);
    }
}

impl Mul for BigInt {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self.mul_with(&other)
    }
}

impl<'a> Mul<&'a Self> for BigInt {
    type Output = Self;

    fn mul(self, other: &'a Self) -> Self::Output {
        self.mul_with(other)
    }
}

impl<'a> Mul<&'a BigInt> for &BigInt {
    type Output = BigInt;

    fn mul(self, other: &'a BigInt) -> Self::Output {
        self.mul_with(other)
    }
}

impl MulAssign for BigInt {
    fn mul_assign(&mut self, other: Self) {
        self.mul_into(&other)
    }
}

impl<'a> MulAssign<&'a Self> for BigInt {
    fn mul_assign(&mut self, other: &'a Self) {
        self.mul_into(other)
    }
}

impl Div for BigInt {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        self.div_by(&other)
    }
}

impl<'a> Div<&'a Self> for BigInt {
    type Output = Self;

    fn div(self, other: &'a Self) -> Self::Output {
        self.div_by(other)
    }
}

impl<'a> Div<&'a BigInt> for &BigInt {
    type Output = BigInt;

    fn div(self, other: &'a BigInt) -> Self::Output {
        self.div_by(other)
    }
}

impl DivAssign for BigInt {
    fn div_assign(&mut self, other: Self) {
        self.div_into(&other);
    }
}

impl<'a> DivAssign<&'a Self> for BigInt {
    fn div_assign(&mut self, other: &'a Self) {
        self.div_into(other);
    }
}
