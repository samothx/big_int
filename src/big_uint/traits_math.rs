use std::ops::{ AddAssign, Add, SubAssign, Sub, MulAssign, Mul, DivAssign, Div};
use super::BigUInt;

impl Add for BigUInt {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.add_to(&other)
    }
}

impl<'a> Add<&'a Self> for BigUInt {
    type Output = Self;

    fn add(self, other: &'a Self) -> Self::Output {
        self.add_to(other)
    }
}

impl<'a> Add<&'a BigUInt> for &BigUInt {
    type Output = BigUInt;

    fn add(self, other: &'a BigUInt) -> Self::Output {
        self.add_to(other)
    }
}

impl AddAssign for BigUInt {
    fn add_assign(&mut self, other: Self) {
        self.add_into(&other);
    }
}

impl<'a> AddAssign<&'a Self> for BigUInt {
    fn add_assign(&mut self, other: &'a Self) {
        self.add_into(other);
    }
}

impl Sub for BigUInt {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.sub_from(&other)
    }
}

impl<'a> Sub<&'a Self> for BigUInt {
    type Output = Self;

    fn sub(self, other: &'a Self) -> Self::Output {
        self.sub_from(&other)
    }
}

impl<'a> Sub<&'a BigUInt> for &BigUInt {
    type Output = BigUInt;

    fn sub(self, other: &'a BigUInt) -> Self::Output {
        self.sub_from(other)
    }
}

impl SubAssign for BigUInt {
    fn sub_assign(&mut self, other: Self) {
        self.sub_into(&other);
    }
}

impl<'a> SubAssign<&'a Self> for BigUInt {
    fn sub_assign(&mut self, other: &'a Self) {
        self.sub_into(other);
    }
}

impl Mul for BigUInt {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self.mul_with(&other)
    }
}

impl<'a> Mul<&'a Self> for BigUInt {
    type Output = Self;

    fn mul(self, other: &'a Self) -> Self::Output {
        self.mul_with(other)
    }
}

impl<'a> Mul<&'a BigUInt> for &BigUInt {
    type Output = BigUInt;

    fn mul(self, other: &'a BigUInt) -> Self::Output {
        self.mul_with(other)
    }
}

impl MulAssign for BigUInt {
    fn mul_assign(&mut self, other: Self) {
        self.mul_into(&other)
    }
}

impl<'a> MulAssign<&'a Self> for BigUInt {
    fn mul_assign(&mut self, other: &'a Self) {
        self.mul_into(other)
    }
}

impl Div for BigUInt {
    type Output = Self;
    fn div(self, other: Self) -> BigUInt {
        let (res, _) = self.div_mod(&other);
        res
    }
}

impl<'a> Div<&'a Self> for BigUInt {
    type Output = Self;
    fn div(self, other: &'a Self) -> BigUInt {
        let (res, _) = self.div_mod(other);
        res
    }
}

impl<'a> Div<&'a BigUInt> for &BigUInt {
    type Output = BigUInt;
    fn div(self, other: &'a BigUInt) -> BigUInt {
        let (res, _) = self.div_mod(other);
        res
    }
}

impl DivAssign for BigUInt {
    fn div_assign(&mut self, other: Self) {
        let _ = self.div_mod_into(&other);
    }
}

impl<'a> DivAssign<&'a Self> for BigUInt {
    fn div_assign(&mut self, other: &'a Self) {
        let _ = self.div_mod_into(other);
    }
}
