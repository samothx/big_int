use std::ops::{ AddAssign, Add, SubAssign, Sub, MulAssign, Mul, DivAssign, Div};
use super::BigUInt;

impl Add for BigUInt {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.add_to(&other)
    }
}

impl AddAssign for BigUInt {
    fn add_assign(&mut self, other: Self) {
        self.add_into(&other);
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
        self.sub_into(&other);
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
        self.mul_into(&other)
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
        let _ = self.div_mod_into(&other);
    }
}
