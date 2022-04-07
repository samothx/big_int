use crate::BigUInt;

#[cfg(test)]
mod test;

mod traits_std;
pub use traits_std::*;

mod traits_math;
pub use traits_math::*;

mod math;
pub use math::*;


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
