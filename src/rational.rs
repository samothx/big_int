use crate::BigUInt;

#[cfg(test)]
mod test;

mod traits_std;
pub use traits_std::*;

mod traits_math;
pub use traits_math::*;

pub struct Rational {
    signed: bool,
    numerator: BigUInt,
    denominator: BigUInt,
}

impl Rational {
    pub fn new() -> Self {
        Self{
            signed: false,
            numerator: BigUInt::new(),
            denominator: 1u64.into()
        }
    }

    pub fn invert(&self) -> Rational {
        Rational{
            signed: self.signed,
            numerator: self.denominator.clone(),
            denominator: self.numerator.clone()
        }
    }

    pub fn invert_into(&mut self) {
        std::mem::swap(&mut self.numerator, &mut self.denominator);
    }

    pub fn mul_by(&self, other: &Self) -> Rational {
        let mut res = Rational{
            signed: self.signed ^ other.signed,
            numerator: self.numerator.mul_with(&other.numerator),
            denominator: self.denominator.mul_with(&other.denominator),
        };

        let gcd = res.numerator.gcd(&res.denominator);
        if gcd > 1u32.into() {
            res.numerator.div_mod_into(&gcd);
            res.denominator.div_mod_into(&gcd);
        }
        res
    }

    pub fn mul_by_into(&mut self, other: &Self) {
        self.signed = self.signed ^ other.signed;
        self.numerator.mul_into(&other.numerator);
        self.denominator.mul_into(&other.denominator);
        let gcd = self.numerator.gcd(&self.denominator);
        if gcd > 1u32.into() {
            self.numerator.div_mod_into(&gcd);
            self.denominator.div_mod_into(&gcd);
        }
    }
}

