use std::ops::{ AddAssign, Add, SubAssign, Sub, MulAssign, Mul, DivAssign, Div};

use super::Rational;

impl Add for Rational {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.add_to(&other)
    }
}

impl<'a> Add<&'a Self> for Rational {
    type Output = Self;

    fn add(self, other: &'a Self) -> Self::Output {
        self.add_to(other)
    }
}

impl<'a> Add<&'a Rational> for &Rational {
    type Output = Rational;

    fn add(self, other: &'a Rational) -> Self::Output {
        self.add_to(other)
    }
}

impl AddAssign for Rational {
    fn add_assign(&mut self, other: Self) {
        self.add_into(&other);
    }
}

impl<'a> AddAssign<&'a Self> for Rational {
    fn add_assign(&mut self, other: &'a Self) {
        self.add_into(&other);
    }
}

impl Sub for Rational {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        self.sub_from(&other)
    }
}

impl<'a> Sub<&'a Self> for Rational {
    type Output = Self;
    fn sub(self, other: &'a Self) -> Self::Output {
        self.sub_from(other)
    }
}

impl<'a> Sub<&'a Rational> for &Rational {
    type Output = Rational;
    fn sub(self, other: &'a Rational) -> Self::Output {
        self.sub_from(other)
    }
}

impl SubAssign for Rational {
    fn sub_assign(&mut self, other: Self) {
        self.sub_into(&other)
    }
}

impl<'a> SubAssign<&'a Self> for Rational {
    fn sub_assign(&mut self, other: &'a Self) {
        self.sub_into(other)
    }
}

impl Mul<Self> for Rational {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self.mul_by(&other)
    }
}

impl<'a> Mul<&'a Self> for Rational {
    type Output = Self;

    fn mul(self, other: &Self) -> Self::Output {
        self.mul_by(other)
    }
}

impl<'a> Mul<&'a Self> for &Rational {
    type Output = Rational;

    fn mul(self, other: &Self) -> Self::Output {
        self.mul_by(other)
    }
}

impl MulAssign for Rational {
    fn mul_assign(&mut self, other: Self) {
        self.mul_into(&other)
    }
}

impl<'a> MulAssign<&'a Self> for Rational {
    fn mul_assign(&mut self, other: &Self) {
        self.mul_into(other)
    }
}

impl Div for Rational {
    type Output = Self;
    fn div(self, other: Self) -> Rational {
        self.div_by(&other)
    }
}

impl<'a> Div<&'a Self> for Rational {
    type Output = Self;
    fn div(self, other: &'a Self) -> Rational {
        self.div_by(other)
    }
}

impl<'a> Div<&'a Rational> for &Rational {
    type Output = Rational;
    fn div(self, other: &'a Rational) -> Rational {
        self.div_by(other)
    }
}

impl DivAssign for Rational {
    fn div_assign(&mut self, other: Self) {
        self.div_into(&other)
    }
}

impl<'a> DivAssign<&'a Self> for Rational {
    fn div_assign(&mut self, other: &'a Self) {
        self.div_into(other)
    }
}
