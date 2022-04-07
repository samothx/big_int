use crate::{Rational, BigInt, BigUInt};
use std::fmt::{Debug, Formatter, Display};

impl From<(u32, u32)> for Rational {
    fn from(src: (u32, u32)) -> Self {
        Self{
            signed: false,
            numerator: src.0.into(),
            denominator: src.1.into()
        }
    }
}

impl From<(i32, i32)> for Rational {
    fn from(src: (i32, i32)) -> Self {
        Self{
            signed: src.0.is_negative() ^ src.1.is_negative(),
            numerator: BigInt::from(src.0).as_unsigned(),
            denominator: BigInt::from(src.1).as_unsigned(),
        }
    }
}

impl From<(u64, u64)> for Rational {
    fn from(src: (u64, u64)) -> Self {
        Self{
            signed: false,
            numerator: src.0.into(),
            denominator: src.1.into()
        }
    }
}

impl From<(i64, i64)> for Rational {
    fn from(src: (i64, i64)) -> Self {
        Self{
            signed: src.0.is_negative() ^ src.1.is_negative(),
            numerator: BigInt::from(src.0).as_unsigned(),
            denominator: BigInt::from(src.1).as_unsigned(),
        }
    }
}

impl From<(u128, u128)> for Rational {
    fn from(src: (u128, u128)) -> Self {
        Self{
            signed: false,
            numerator: src.0.into(),
            denominator: src.1.into()
        }
    }
}

impl From<(i128, i128)> for Rational {
    fn from(src: (i128, i128)) -> Self {
        Self{
            signed: src.0.is_negative() ^ src.1.is_negative(),
            numerator: BigInt::from(src.0).as_unsigned(),
            denominator: BigInt::from(src.1).as_unsigned(),
        }
    }
}

impl From<(BigUInt, BigUInt)> for Rational {
    fn from(src: (BigUInt, BigUInt)) -> Self {
        Self{
            signed: false,
            numerator: src.0,
            denominator: src.1
        }
    }
}

impl From<(BigInt, BigInt)> for Rational {
    fn from(src: (BigInt, BigInt)) -> Self {
        Self{
            signed: src.0.is_negative() ^ src.1.is_negative(),
            numerator: src.0.as_unsigned(),
            denominator: src.1.as_unsigned(),
        }
    }
}

impl Debug for Rational {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_integer() {
            write!(f, "({}{:?})", if self.signed { "-" } else { "" }, self.numerator)
        } else {
            write!(f, "({}{:?}/{:?})", if self.signed { "-" } else { "" }, self.numerator, self.denominator)
        }
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_integer() {
            write!(f, "{}{}", if self.signed { "-" } else { "" }, self.numerator)
        } else {
            write!(f, "{}{}/{}", if self.signed { "-" } else { "" }, self.numerator, self.denominator)
        }
    }
}
