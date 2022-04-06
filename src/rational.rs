use crate::BigUInt;

mod test;
mod traits;
pub use traits::*;

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
}

