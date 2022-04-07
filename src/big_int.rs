use crate::BigUInt;

#[cfg(test)]
mod test;

mod traits;
pub use traits::*;
use std::cmp::Ordering;

/// A signed integer of indefinite size, limited only by memory constraints and rust maximum
/// vector size.
#[derive(Clone, PartialEq)]
pub struct BigInt {
    signed: bool,
    uint: BigUInt
}

impl BigInt {
    /// Create an empty (zero value) BigUInt.
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigInt;
    /// let bi = BigInt::new();
    /// ```
    #[inline]
    pub fn new() -> BigInt {
        BigInt {
            signed: false,
            uint: BigUInt::new(),
        }
    }


    /// Create a BigInt from an i8 value.
    ///
    /// # Arguments
    ///
    /// * from - the i8 value
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigInt;
    /// let bi = BigInt::from_i8(0x7F);
    /// assert_eq!(bi.to_i64(),Some(0x7F));
    /// ```
    #[inline]
    pub fn from_i8(from: i8) -> BigInt {
        BigInt {
            signed: if from >= 0 { false } else { true },
            uint: BigUInt::from_u8(i8::abs(from) as u8)
        }
    }

    /// Create a BigInt from an i16 value.
    ///
    /// # Arguments
    ///
    /// * from - the i16 value
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigInt;
    /// let bi = BigInt::from_i16(0x7FFF);
    /// assert_eq!(bi.to_i64(),Some(0x7FFF));
    /// ```
    #[inline]
    pub fn from_i16(from: i16) -> BigInt {
        BigInt {
            signed: if from >= 0 { false } else { true },
            uint: BigUInt::from_u16(i16::abs(from) as u16)
        }
    }


    /// Create a BigUInt from an i32 value.
    ///
    /// # Arguments
    ///
    /// * from - the i32 value
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigInt;
    /// let bi = BigInt::from_i32(0x7FFFFFFF);
    /// assert_eq!(bi.to_i64(),Some(0x7FFFFFFF));
    /// ```
    #[inline]
    pub fn from_i32(from: i32) -> BigInt {
        BigInt {
            signed: if from >= 0 { false } else { true },
            uint: BigUInt::from_u32(i32::abs(from) as u32)
        }
    }

    /// Create a BigUInt from an i64 value.
    ///
    /// # Arguments
    ///
    /// * from - the i64 value
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigInt;
    /// let bi = BigInt::from_i64(0x7FFFFFFFFFFFFFFF);
    /// assert_eq!(bi.to_i64(),Some(0x7FFFFFFFFFFFFFFF));
    /// ```
    #[inline]
    pub fn from_i64(from: i64) -> BigInt {
        if from as u64 == 0xFFFFFFFFFFFFFFFF {
            BigInt {
                signed: true,
                uint: BigUInt::from_u64(1<<63)
            }
        } else {
            BigInt {
                signed: if from >= 0 { false } else { true },
                uint: BigUInt::from_u64(i64::abs(from) as u64)
            }
        }
    }

    /// Create a BigUInt from an i128 value.
    ///
    /// # Arguments
    ///
    /// * from - the i128 value
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigInt;
    /// let bi = BigInt::from_i128(0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);
    /// assert_eq!(bi.to_i128(),Some(0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF));
    /// ```
    #[inline]
    pub fn from_i128(from: i128) -> BigInt {
        if from as u128 == 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF {
            BigInt {
                signed: true,
                uint: BigUInt::from_u128(1<<127)
            }
        } else {
            BigInt {
                signed: if from >= 0 { false } else { true },
                uint: BigUInt::from_u128(i128::abs(from) as u128)
            }
        }
    }

    /// Create an i64 from a BigInt value.
    ///
    /// # Returns
    /// An option returning the i64 value or None if the BigInt was larger then the maximum value for i64
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigInt;
    /// let bi = BigInt::from_i64(0x7FFFFFFFFFFFFFFF);
    /// assert_eq!(bi.to_i64(),Some(0x7FFFFFFFFFFFFFFF));
    /// ```
    #[inline]
    pub fn to_i64(&self) -> Option<i64> {
        if self.uint.is_zero() {
            Some(0)
        } else if self.uint.length() > 63 {
            None
        } else {
            let uint = self.uint.to_u64().expect("unexpected uint > 63 bit");
            if self.signed {
                Some((self.uint.to_u64().expect("unexpected oversize uint") as i64) * -1)
            } else {
                if uint < 1 << 63 {
                    Some(self.uint.to_u64().expect("unexpected oversize uint") as i64)
                } else {
                    None
                }
            }
        }
    }

    /// Create an i64 from a BigInt value.
    ///
    /// # Returns
    /// An option returning the i64 value or None if the BigInt was larger then the maximum value for i64
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigInt;
    /// let bi = BigInt::from_i128(0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);
    /// assert_eq!(bi.to_i128(),Some(0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF));
    /// ```
    #[inline]
    pub fn to_i128(&self) -> Option<i128> {
        if self.uint.is_zero() {
            Some(0)
        } else if self.uint.length() > 127 {
            None
        } else {
            let uint = self.uint.to_u128().expect("unexpected uint > 128 bit");
            if self.signed {
                Some((self.uint.to_u128().expect("unexpected oversize uint") as i128) * -1)
            } else {
                if uint < (1 << 127) {
                    Some(self.uint.to_u128().expect("unexpected oversize uint") as i128)
                } else {
                    None
                }
            }
        }
    }

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
    /// bi1.add_to_self(&bi2);
    /// assert_eq!(bi1.to_dec_str(),"30");
    /// ```
    #[inline]
    pub fn add_to_self(&mut self, other: &Self) {
        if self.signed == other.signed {
            // same sign - just add & keep sign
            self.uint.add_to_self(&other.uint);
        } else if self.signed {
            // other is positive so -a + b -> b - a
            *self = other.sub_from_unsigned(self);
        } else {
            // self is positive so a - b
            self.sub_from_self_unsigned(&other);
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
    fn sub_from_self_unsigned(&mut self, other: &Self) {
        match (self.uint).cmp(&other.uint) {
            Ordering::Greater => {
                self.uint.sub_from_self(&other.uint);
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
    pub fn sub_from_self(&mut self, other: &Self) {
        if self.signed == other.signed {
            self.sub_from_self_unsigned(other)
        } else  {
            self.uint.add_to_self(&other.uint)
        }
    }


    /// Return the value of the BigInt as a decimal string
    ///
    /// # Returns
    /// The value of the BigInt as a decimal string
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigInt;
    /// let mut bi1 = BigInt::from(10);
    /// assert_eq!(bi1.to_dec_str(),"10");
    /// ```
    #[inline]
    pub fn to_dec_str(&self) -> String {
        format!("{}{}", if self.signed { "-" } else { "" }, self.uint.to_dec_string())
    }

    #[inline]
    pub fn mul_with(&self, other: &Self) -> Self {
        BigInt {
            signed: self.signed ^ other.signed,
            uint: self.uint.mul_with(&other.uint)
        }
    }

    #[inline]
    pub fn mul_with_self(&mut self, other: &Self) {
        self.signed = self.signed ^ other.signed;
        self.uint.mul_with_self(&other.uint);
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

    #[inline]
    pub fn is_positive(&self) -> bool {
        !self.signed
    }

    #[inline]
    pub fn is_negative(&self) -> bool {
        self.signed
    }

    #[inline]
    pub fn set_positive(&mut self) {
        self.signed = false;
    }

    #[inline]
    pub fn set_negative(&mut self) {
        self.signed = true;
    }

    #[inline]
    pub fn reverse_sign(&mut self) {
        self.signed = !self.signed;
    }

    #[inline]
    pub fn abs(&self) -> BigInt {
        BigInt{
            signed: false,
            uint: self.uint.clone()
        }
    }

    #[inline]
    pub fn as_unsigned(self) -> BigUInt {
        self.uint
    }



}
