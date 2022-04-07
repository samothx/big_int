use std::ops::{ AddAssign, Add, SubAssign, Sub, MulAssign, Mul, DivAssign, Div};

use super::Rational;

impl Add for Rational {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        todo!()
    }
}

impl AddAssign for Rational {
    fn add_assign(&mut self, other: Self) {
        todo!()
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        todo!()
    }
}

impl SubAssign for Rational {
    fn sub_assign(&mut self, other: Self) {
        todo!()
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self.mul_by(&other)
    }
}

impl MulAssign for Rational {
    fn mul_assign(&mut self, other: Self) {
        self.mul_by_into(&other)
    }
}

impl Div for Rational {
    type Output = Self;
    fn div(self, other: Self) -> Rational {
        let register = self.invert();
        register.mul(other)
    }
}

impl DivAssign for Rational {
    fn div_assign(&mut self, other: Self) {
        self.invert_into();
        self.mul_by_into(&other);
    }
}
