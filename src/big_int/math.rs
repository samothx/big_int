use crate::{BigInt, BigUInt};
use std::cmp::Ordering;

impl BigInt {
    /// Add one BigInt to another
    ///
    /// Due to BigInt not being able to implement the Copy trait and the std::ops::Add trait
    /// consuming the right hand side operator, the use of + can be inefficient having to clone
    /// the right hand side operator.
    /// This function works around that restriction, it is used by the std::ops::Add implementation
    ///
    /// # Returns
    /// The result of the addition as BigInt
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigInt;
    /// let bi1 = BigInt::from(10);
    /// let bi2 = BigInt::from(20);
    /// assert_eq!(bi1.add_to(&bi2).to_dec_str(),"30");
    /// ```
    #[inline]
    pub fn add_to(&self, other: &Self) -> BigInt {
        if self.signed == other.signed {
            // both same signed, just add & keep sign
            Self {
                signed: self.signed,
                uint: self.uint.add_to(&other.uint),
            }
        } else if self.signed {
            // other is positive so -a + b -> b - a
            other.sub_from_unsigned(&self)
        } else {
            // self is positive so a - b
            self.sub_from_unsigned(&other)
        }
    }

    /// Add one BigInt to another and store the result in self
    ///
    /// Due to BigInt not being able to implement the Copy trait and the std::ops::AddAssign trait
    /// consuming the right hand side operator, the use of += can be inefficient having to clone
    /// the right hand side operator.
    /// This function works around that restriction, it is used by the std::ops::AddAssign implementation
    ///
    /// # Returns
    /// The result of the addition in self
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigInt;
    /// let mut bi1 = BigInt::from(10);
    /// let bi2 = BigInt::from(20);
    /// bi1.add_into(&bi2);
    /// assert_eq!(bi1.to_dec_str(),"30");
    /// ```
    #[inline]
    pub fn add_into(&mut self, other: &Self) {
        if self.signed == other.signed {
            // same sign - just add & keep sign
            self.uint.add_into(&other.uint);
        } else if self.signed {
            // other is positive so -a + b -> b - a
            *self = other.sub_from_unsigned(self);
        } else {
            // self is positive so a - b
            self.sub_from_into_unsigned(&other);
        }
    }

    #[inline]
    fn sub_from_unsigned(&self, other: &Self) -> BigInt {
        if self.uint > other.uint {
            BigInt{
                signed: self.signed,
                uint: self.uint.sub_from(&other.uint)
            }
        }  else if self.uint < other.uint {
            // sign reversal
            BigInt{
                signed: !self.signed,
                uint: other.uint.sub_from(&self.uint)
            }
        } else {
            BigInt::new()
        }
    }

    /// Subtract one BigInt from another
    ///
    /// Due to BigInt not being able to implement the Copy trait and the std::ops::Sub trait
    /// consuming the right hand side operator, the use of - can be inefficient having to clone
    /// the right hand side operator.
    /// This function works around that restriction, it is used by the std::ops::Sub implementation
    ///
    /// # Returns
    /// The result of the subtraction as BigInt
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigInt;
    /// let bi1 = BigInt::from(10);
    /// let bi2 = BigInt::from(20);
    /// assert_eq!(bi1.sub_from(&bi2).to_dec_str(),"-10");
    /// ```
    #[inline]
    pub fn sub_from(&self, other: &Self) -> BigInt {
        if self.signed == other.signed {
            self.sub_from_unsigned(other)
        } else  {
            BigInt{
                signed: self.signed,
                uint: self.uint.add_to(&other.uint)
            }
        }
    }

    #[inline]
    fn sub_from_into_unsigned(&mut self, other: &Self) {
        match (self.uint).cmp(&other.uint) {
            Ordering::Greater => {
                self.uint.sub_into(&other.uint);
            }
            Ordering::Less => {
                // sign reversal
                self.signed = !self.signed;
                self.uint =  other.uint.sub_from(&self.uint)
            },
            Ordering::Equal => {
                self.signed = false;
                self.uint = BigUInt::new()
            }
        }
    }

    /// Subtract one BigInt from another and store the result in self
    ///
    /// Due to BigInt not being able to implement the Copy trait and the std::ops::SubAssign trait
    /// consuming the right hand side operator, the use of -= can be inefficient having to clone
    /// the right hand side operator.
    /// This function works around that restriction, it is used by the std::ops::SubAssign implementation
    ///
    /// # Returns
    /// The result of the subtraction in self
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigInt;
    /// let mut bi1 = BigInt::from(10);
    /// let bi2 = BigInt::from(20);
    /// bi1 -= bi2;
    /// assert_eq!(bi1.to_dec_str(),"-10");
    /// ```
    #[inline]
    pub fn sub_into(&mut self, other: &Self) {
        if self.signed == other.signed {
            self.sub_from_into_unsigned(other)
        } else  {
            self.uint.add_into(&other.uint)
        }
    }

    #[inline]
    pub fn mul_with(&self, other: &Self) -> Self {
        BigInt {
            signed: self.signed ^ other.signed,
            uint: self.uint.mul_with(&other.uint)
        }
    }

    #[inline]
    pub fn mul_into(&mut self, other: &Self) {
        self.signed = self.signed ^ other.signed;
        self.uint.mul_into(&other.uint);
    }

    pub fn pow(&self, power: u32) -> BigInt {
        BigInt {
            signed: if self.signed {
                (power & 0x1) == 0x1
            } else {
                false
            },
            uint: self.uint.pow(power)
        }
    }

    pub fn to_f64(&self) -> f64 {
        if self.signed {
            -self.uint.to_f64()
        } else {
            self.uint.to_f64()
        }
    }

}
