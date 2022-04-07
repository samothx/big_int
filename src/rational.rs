use crate::BigUInt;

#[cfg(test)]
mod test;

mod traits_std;

pub use traits_std::*;

mod traits_math;

pub use traits_math::*;

mod math;
pub use math::*;

pub struct Rational {
    signed: bool,
    numerator: BigUInt,
    denominator: BigUInt,
}

impl Rational {
    pub fn new() -> Self {
        Self {
            signed: false,
            numerator: BigUInt::new(),
            denominator: 1u64.into(),
        }
    }

    #[inline]
    pub fn is_positive(&self) -> bool {
        !self.signed
    }

    #[inline]
    pub fn is_negative(&self) -> bool {
        self.signed
    }

    #[inline]
    pub fn is_integer(&self) -> bool {
        self.denominator == 1u32.into()
    }

    pub fn to_f64(&self) -> f64 {
        if self.signed {
            -self.numerator.to_f64() / self.denominator.to_f64()
        } else {
            self.numerator.to_f64() / self.denominator.to_f64()
        }
    }
}

impl<'a> Rational {
    pub fn numerator(&'a self) -> &'a BigUInt {
        &self.numerator
    }

    pub fn denominator(&'a self) -> &'a BigUInt {
        &self.denominator
    }
}
