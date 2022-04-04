use crate::BigUInt;

/// An unsigned integer of indefinite size, limited only by memory constraints and rust maximum
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
    /// let bi = BigInt::from_i8(0xFF);
    /// assert_eq!(bi.to_hex_string(),"FF");
    /// ```
    pub fn from_i8(from: i8) -> BigInt {
        BigInt {
            signed: if from > 0 { false } else { true },
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
    /// let bi = BigInt::from_i16(0xFFFF);
    /// assert_eq!(bi.to_hex_string(),"FFFF");
    /// ```
    pub fn from_i16(from: i16) -> BigInt {
        BigInt {
            signed: if from > 0 { false } else { true },
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
    /// let bi = BigInt::from_i32(0xFFFFFFFF);
    /// assert_eq!(bi.to_hex_string(),"FFFFFFFF");
    /// ```

    pub fn from_i32(from: i32) -> BigInt {
        BigInt {
            signed: if from > 0 { false } else { true },
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
    /// let bi = BigInt::from_i64(0xFFFFFFFFFFFFFFFF);
    /// assert_eq!(bi.to_hex_string(),"FFFFFFFFFFFFFFFF");
    /// ```

    pub fn from_i64(from: i64) -> BigInt {
        BigInt {
            signed: if from > 0 { false } else { true },
            uint: BigUInt::from_u64(i64::abs(from) as u64)
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
    /// let bi = BigInt::from_i128(0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);
    /// assert_eq!(bi.to_hex_string(),"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
    /// ```

    pub fn from_i128(from: i128) -> BigInt {
        BigInt {
            signed: if from > 0 { false } else { true },
            uint: BigUInt::from_u128(i128::abs(from) as u128)
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
    /// let bi = BigInt::from_u64(0xFFFFFFFFFFFFFFFF);
    /// assert_eq!(bi.to_i64(),Some(0xFFFFFFFFFFFFFFFF));
    /// ```
    pub fn to_i64(&self) -> Option<i64> {
        if self.uint.is_empty() {
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
}
