use crate::{Rational, BigUInt, BigInt};
use std::fmt::{Debug, Formatter, Display};
use std::cmp::{Ordering, PartialOrd, Eq};

impl Eq for Rational {}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        // TODO: prerequisite rational is always gcd reduced
        self.signed == other.signed &&
            self.numerator == other.numerator &&
            self.denominator == other.denominator
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.signed == other.signed {
            let diff = self.abs().sub_from(&other.abs());
            if diff.is_zero() {
                Ordering::Equal
            } else if diff.is_positive() {
                if self.signed { Ordering::Less } else { Ordering::Greater }
            } else {
                if self.signed { Ordering::Greater } else { Ordering::Less }
            }
        } else if self.signed {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}


impl From<(u32, u32)> for Rational {
    fn from(src: (u32, u32)) -> Self {
        if src.1 == 0 {
            panic!("division by zero");
        } else {
            Self {
                signed: false,
                numerator: src.0.into(),
                denominator: src.1.into(),
            }
        }
    }
}

impl From<(i32, i32)> for Rational {
    fn from(src: (i32, i32)) -> Self {
        if src.1 == 0 {
            panic!("division by zero");
        } else {
            Self {
                signed: src.0.is_negative() ^ src.1.is_negative(),
                numerator: BigInt::from(src.0).as_unsigned(),
                denominator: BigInt::from(src.1).as_unsigned(),
            }
        }
    }
}

impl From<(u64, u64)> for Rational {
    fn from(src: (u64, u64)) -> Self {
        if src.1 == 0 {
            panic!("division by zero");
        } else {
            Self {
                signed: false,
                numerator: src.0.into(),
                denominator: src.1.into(),
            }
        }
    }
}


impl From<(i64, i64)> for Rational {
    fn from(src: (i64, i64)) -> Self {
        if src.1 == 0 {
            panic!("division by zero");
        } else {
            Self {
                signed: src.0.is_negative() ^ src.1.is_negative(),
                numerator: BigInt::from(src.0).as_unsigned(),
                denominator: BigInt::from(src.1).as_unsigned(),
            }
        }
    }
}

impl From<(u128, u128)> for Rational {
    fn from(src: (u128, u128)) -> Self {
        if src.1 == 0 {
            panic!("division by zero");
        } else {
            Self {
                signed: false,
                numerator: src.0.into(),
                denominator: src.1.into(),
            }
        }
    }
}

impl From<(i128, i128)> for Rational {
    fn from(src: (i128, i128)) -> Self {
        if src.1 == 0 {
            panic!("division by zero");
        } else {
            Self {
                signed: src.0.is_negative() ^ src.1.is_negative(),
                numerator: BigInt::from(src.0).as_unsigned(),
                denominator: BigInt::from(src.1).as_unsigned(),
            }
        }
    }
}

impl From<i32> for Rational {
    fn from(src: i32) -> Self {
        Self {
            signed: src.is_negative(),
            numerator: BigInt::from(src).as_unsigned(),
            denominator: BigUInt::from(1u32),
        }
    }
}

impl From<u32> for Rational {
    fn from(src: u32) -> Self {
        Self {
            signed: false,
            numerator: BigUInt::from(src),
            denominator: BigUInt::from(1u32),
        }
    }
}

impl From<i64> for Rational {
    fn from(src: i64) -> Self {
        Self {
            signed: src.is_negative(),
            numerator: BigInt::from(src).as_unsigned(),
            denominator: BigUInt::from(1u32),
        }
    }
}

impl From<u64> for Rational {
    fn from(src: u64) -> Self {
        Self {
            signed: false,
            numerator: BigUInt::from(src),
            denominator: BigUInt::from(1u32),
        }
    }
}

impl From<i128> for Rational {
    fn from(src: i128) -> Self {
            Self {
                signed: src.is_negative(),
                numerator: BigInt::from(src).as_unsigned(),
                denominator: BigUInt::from(1u32),
            }
    }
}

impl From<u128> for Rational {
    fn from(src: u128) -> Self {
        Self {
            signed: false,
            numerator: BigUInt::from(src),
            denominator: BigUInt::from(1u32),
        }
    }
}

impl From<(BigUInt, BigUInt)> for Rational {
    fn from(src: (BigUInt, BigUInt)) -> Self {
        if src.1.is_zero() {
            panic!("division by zero");
        } else {
            Self {
                signed: false,
                numerator: src.0,
                denominator: src.1,
            }
        }
    }
}

impl From<BigUInt> for Rational {
    fn from(src: BigUInt) -> Self {
        Self {
            signed: false,
            numerator: src,
            denominator: 1u32.into(),
        }
    }
}

impl From<(BigInt, BigInt)> for Rational {
    fn from(src: (BigInt, BigInt)) -> Self {
        Self {
            signed: src.0.is_negative(),
            numerator: src.0.as_unsigned(),
            denominator: src.1.as_unsigned(),
        }
    }
}

impl From<BigInt> for Rational {
    fn from(src: BigInt) -> Self {
        Self {
            signed: src.is_negative(),
            numerator: src.as_unsigned(),
            denominator: 1u32.into(),
        }
    }
}

impl From<f64> for Rational {
    fn from(src: f64) -> Self {
        Self::from_f64(src)
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
