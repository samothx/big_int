type Block = u64;

const BLOCK_SIZE: usize = 64;
const BLOCK_MASK: u64 = 0xFFFFFFFFFFFFFFFF;
const BIT_64: u64 = 0x8000000000000000;
const BIT_65: u128 = 0x10000000000000000;

const HEX_DIGITS: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

mod traits_std;

pub use traits_std::*;

mod traits_math;

pub use traits_math::*;

mod traits_bit;

pub use traits_bit::*;

mod math;

pub use math::*;

mod bits;
pub use bits::*;

#[cfg(test)]
mod test;

/// An unsigned integer of indefinite size, limited only by memory constraints and rust maximum
/// vector size.
#[derive(Clone, PartialEq)]
pub struct BigUInt {
    length: usize,
    bits: Vec<Block>,
}

impl BigUInt {
    /// Create an empty (zero value) BigUInt.
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let bi = BigUInt::new();
    /// ```
    #[inline]
    pub fn new() -> BigUInt {
        BigUInt {
            length: 0,
            bits: Vec::new(),
        }
    }

    pub fn from_hex_str(src: &str) -> Result<BigUInt, String> {
        let mut bits = Vec::new();
        let mut register = 0u64;
        for (idx, chr) in src.chars().rev().enumerate() {
            if chr >= 'A' && chr <= 'F' {
                register += (chr as u64 - 'A' as u64 + 10) << ((idx % 16) * 4);
            } else if chr >= 'a' && chr <= 'f' {
                register += (chr as u64 - 'a' as u64 + 10) << ((idx % 16) * 4);
            } else if chr >= '0' && chr <= '9' {
                register += (chr as u64 - '0' as u64) << ((idx % 16) * 4);
            } else {
                return Err(format!("invalid character encountered in hex string: '{}'", chr));
            }

            if idx % 16 == 15 {
                bits.push(register);
                register = 0;
            }
        }

        if register > 0 {
            bits.push(register);
        }

        let mut res = BigUInt {
            length: bits.len() * BLOCK_SIZE,
            bits,
        };
        res.trim();

        Ok(res)
    }

    /// Create a BigUInt from an u8 value.
    ///
    /// # Arguments
    ///
    /// * from - the u8 value
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let bi = BigUInt::from_u8(0xFF);
    /// assert_eq!(bi.to_hex_string(),"FF");
    /// ```
    #[inline]
    pub fn from_u8(from: u8) -> BigUInt {
        if from > 0 {
            let mut work = from;

            let mut length = 0usize;
            for idx in (1..=8).rev() {
                if work & 0x80 == 0x80 {
                    length = idx;
                    break;
                } else {
                    work <<= 1;
                }
            }
            BigUInt {
                length,
                bits: vec![from as Block],
            }
        } else {
            BigUInt::new()
        }
    }

    /// Create a BigUInt from an u16 value.
    ///
    /// # Arguments
    ///
    /// * from - the u16 value
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let bi = BigUInt::from_u16(0xFFFF);
    /// assert_eq!(bi.to_hex_string(),"FFFF");
    /// ```
    #[inline]
    pub fn from_u16(from: u16) -> BigUInt {
        if from > 0 {
            let mut work = from;

            let mut length = 0usize;
            for idx in (1..=16).rev() {
                if work & 0x8000 == 0x8000 {
                    length = idx;
                    break;
                } else {
                    work <<= 1;
                }
            }
            BigUInt {
                length,
                bits: vec![from as Block],
            }
        } else {
            BigUInt::new()
        }
    }

    /// Create a BigUInt from an u32 value.
    ///
    /// # Arguments
    ///
    /// * from - the u32 value
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let bi = BigUInt::from_u32(0xFFFFFFFF);
    /// assert_eq!(bi.to_hex_string(),"FFFFFFFF");
    /// ```
    #[inline]
    pub fn from_u32(from: u32) -> BigUInt {
        if from > 0 {
            let mut work = from;

            let mut length = 0usize;
            for idx in (1..=32).rev() {
                if work & 0x80000000 == 0x80000000 {
                    length = idx;
                    break;
                } else {
                    work <<= 1;
                }
            }
            BigUInt {
                length,
                bits: vec![from as Block],
            }
        } else {
            BigUInt::new()
        }
    }

    /// Create a BigUInt from an u64 value.
    ///
    /// # Arguments
    ///
    /// * from - the u64 value
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let bi = BigUInt::from_u64(0xFFFFFFFFFFFFFFFF);
    /// assert_eq!(bi.to_hex_string(),"FFFFFFFFFFFFFFFF");
    /// ```
    #[inline]
    pub fn from_u64(from: u64) -> BigUInt {
        if from > 0 {
            let mut work = from;

            let mut length = 0usize;
            for idx in (1..=64).rev() {
                if work & 0x8000000000000000 == 0x8000000000000000 {
                    length = idx;
                    break;
                } else {
                    work <<= 1;
                }
            }
            BigUInt {
                length,
                bits: vec![from as Block],
            }
        } else {
            BigUInt::new()
        }
    }

    /// Create a BigUInt from an u128 value.
    ///
    /// # Arguments
    ///
    /// * from - the u128 value
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let bi = BigUInt::from_u128(0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);
    /// assert_eq!(bi.to_hex_string(),"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
    /// ```
    #[inline]
    pub fn from_u128(from: u128) -> BigUInt {
        if from > 0 {
            let (bits, mut high_block) = if from > BLOCK_MASK as u128 {
                let mut bits = vec![(from & BLOCK_MASK as u128) as Block, ];
                let high_block = (from >> BLOCK_SIZE) as Block;
                bits.push(high_block);
                (bits, high_block)
            } else {
                let mut bits = vec![];
                let high_block = from as Block;
                bits.push(high_block);
                (bits, high_block)
            };

            let mut length = 0usize;
            for idx in (1..=BLOCK_SIZE).rev() {
                if high_block & BIT_64 == BIT_64 {
                    length = idx;
                    break;
                } else {
                    high_block <<= 1;
                }
            }
            if bits.len() > 1 {
                length += BLOCK_SIZE
            }
            BigUInt {
                length,
                bits,
            }
        } else {
            BigUInt::new()
        }
    }

    /// Create a u64 from a BigUInt value.
    ///
    /// # Returns
    /// An option returning the u64 value or None if the BigUInt was larger then the maximum value for u64
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let bi = BigUInt::from_u64(0xFFFFFFFFFFFFFFFF);
    /// assert_eq!(bi.to_u64(),Some(0xFFFFFFFFFFFFFFFF));
    /// ```
    #[inline]
    pub fn to_u64(&self) -> Option<u64> {
        if self.is_zero() {
            Some(0)
        } else if self.length < 65 {
            Some(self.bits[0])
        } else {
            None
        }
    }

    /// Create a u128 from a BigUInt value.
    ///
    /// # Returns
    /// An option returning the u64 value or None if the BigUInt was larger then the maximum value for u128
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let bi = BigUInt::from_u128(0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);
    /// assert_eq!(bi.to_u128(),Some(0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF));
    /// ```
    #[inline]
    pub fn to_u128(&self) -> Option<u128> {
        if self.is_zero() {
            Some(0)
        } else if self.length < 129 {
            if self.length > 64 {
                Some(((self.bits[1] as u128) << 64) | self.bits[0] as u128)
            } else {
                Some(self.bits[0] as u128)
            }
        } else {
            None
        }
    }

    /// Retrieve the number of bits stored in the BigUInt
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let bi = BigUInt::from_u32(0xFFFFFFF);
    /// assert_eq!(bi.length(), 28);
    /// ```
    #[inline]
    pub fn length(&self) -> usize {
        self.length
    }

    /// Check if the BigUInt is 0 / empty
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let bi = BigUInt::from_u32(0xFFFFFFF);
    /// assert!(!bi.is_zero());
    /// ```
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.length == 0
    }

    /// Create an iterator that iterates over the bits of a BigUInt and returns each bit value
    /// as true or false.
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let bi = BigUInt::from_u32(0xF0F0F0F0);
    /// let mut res = 0u32;
    /// bi.iter().for_each(|bit| {
    ///     res <<= 1;
    ///     if bit {
    ///         res |= 1;
    ///     }
    /// });
    /// assert_eq!(res, 0xF0F0F0F0)
    /// ```

    pub fn trailing_zeros(&self) -> u32 {
        let mut tz_sum = 0u32;
        for block in &self.bits {
            if *block == 0 {
                tz_sum += BLOCK_SIZE as u32;
            } else {
                tz_sum += block.trailing_zeros();
                break;
            }
        }
        tz_sum
    }


    /// Convert the BigUInt to a string of binary digits
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let mut bi = BigUInt::from_u64(0xF0F0F0F0F0F0F0F0);
    /// assert_eq!(bi.to_bin_string(),"1111000011110000111100001111000011110000111100001111000011110000");
    /// ```

    pub fn to_bin_string(&self) -> String {
        if self.is_zero() {
            String::from('0')
        } else {
            let mut res = String::new();
            for bit in self.iter() {
                if bit {
                    res.push('1');
                } else {
                    res.push('0');
                }
            }
            res
        }
    }

    /// Convert the BigUInt to a string of decimal digits
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let bi = BigUInt::from_u64(0xAB54A98F81652440);
    /// assert_eq!(bi.to_dec_string(), "12345678912345678912");
    /// ```

    pub fn to_dec_string(&self) -> String {
        let divisor = BigUInt::from_u32(10);
        let mut res = vec![];
        let mut work = self.clone();
        while work >= divisor {
            let modulo = work.div_mod_into(&divisor);
            let digit = modulo.to_u64().expect("Unexpected big value in modulo");
            res.push(HEX_DIGITS[digit as usize]);
        }

        if !work.is_zero() {
            let digit = work.to_u64().expect("Unexpected big value in modulo");
            res.push(HEX_DIGITS[digit as usize]);
        }

        if res.is_empty() {
            String::from('0')
        } else {
            let mut str_res = String::new();
            res.iter().rev().for_each(|ch| { str_res.push(*ch); });
            str_res
        }
    }

    /// Convert the BigUInt to a string of hexadecimal digits
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let mut bi = BigUInt::from_u64(0xF0F0F0F0F0F0F0F0);
    /// assert_eq!(bi.to_hex_string(),"F0F0F0F0F0F0F0F0");
    /// ```

    pub fn to_hex_string(&self) -> String {
        //eprintln!("to_hex_string: 0b{}", self.to_bin_string());
        if self.is_zero() {
            //eprintln!("to_hex_string: 0b{} - empty", self.to_bin_string());
            String::from('0')
        } else {
            //eprintln!("to_hex_string: 0b{} - length: {}", self.to_bin_string(), self.length);
            let mut res = String::new();
            let mut digit = 0;
            let offset = self.length - 1;
            for (index, bit) in self.iter().enumerate() {
                digit <<= 1;
                if bit {
                    digit += 1;
                }
                //eprintln!("to_hex_string: 0b{} - loop idx: {}, value: {}, digit: {:x}", self.to_bin_string(), index, bit, digit);
                if (offset - index) % 4 == 0 {
                    res.push(HEX_DIGITS[digit]);
                    digit = 0;
                }
            }
            res
        }
    }

    /// Trim any leading digits that contain no value (no bits set to 1)
    fn trim(&mut self) {
        if self.length > 0 {
            let mut blocks = 0;
            let mut high_block = 0;
            for (idx, block) in self.bits.iter().enumerate().rev() {
                if *block > 0 {
                    blocks = idx + 1;
                    high_block = *block;
                    break;
                }
            }
            if blocks == 0 {
                self.length = 0;
                self.bits.clear();
            } else {
                // TODO: use length to make more efficient
                self.bits.resize(blocks, 0);
                let mut length = 0usize;
                for idx in (1..=64).rev() {
                    if high_block & 0x8000000000000000 == 0x8000000000000000 {
                        length = idx;
                        break;
                    } else {
                        high_block <<= 1;
                    }
                }
                self.length = (self.bits.len() - 1) * BLOCK_SIZE + length;
            }
        }
    }

    #[inline]
    pub fn is_even(&self) -> bool {
        self.is_zero() || ((self.bits[0] & 0x1) == 0)
    }

    #[inline]
    pub fn is_odd(&self) -> bool {
        !self.is_even()
    }
}

