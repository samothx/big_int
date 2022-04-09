use std::cmp::Ordering;
use crate::{macros::function,BigUInt};
use super::{BLOCK_SIZE, BLOCK_MASK};

impl BigUInt {
    /// Get the value of an individual bit
    ///
    /// # Arguments
    /// * index - the index of the bit
    ///
    /// # Return Values
    ///  * Some(true) - if the bit is set
    ///  * Some(false) - if the bit is not set
    ///  * None - if the index was out of range
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let bi = BigUInt::from_u32(0xF0F0F0F0);
    /// assert_eq!(bi.get(31), Some(true))
    /// ```

    pub fn get(&self, index: usize) -> Option<bool> {
        if index >= self.length {
            None
        } else {
            let bit_offset = index % BLOCK_SIZE;
            Some((self.bits[index / BLOCK_SIZE] >> bit_offset) & 1 == 1)
        }
    }


    /// Will extract the requested amount of bits out of self and return them as a BigUInt.
    ///
    /// Due to the way BigUInt is created left trailing zeros are trimmed so that result length may
    /// be less than requested length.
    ///
    /// # Arguments
    /// * start - index of the first bit
    /// * num - number of bits to extract
    ///
    /// Please be aware that the index will count down instead of up, meaning
    /// ```
    /// use simple_big_int::BigUInt;
    /// let bi = BigUInt::from_u128(0xF0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0);
    /// let res = bi.get_bits(127,8);
    /// ```
    /// will get you bits 127..120
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let bi = BigUInt::from_u128(0xF0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0);
    /// let res = bi.get_bits(23,8);
    /// assert_eq!(res.to_hex_string(),"F0");
    /// ```
    pub fn get_bits(&self, start: usize, num_bits: usize) -> BigUInt {
        assert!(start < self.length, "Index out of range, start is too big: start >= length {}>={}", start, self.length);
        assert!(start + 1 >= num_bits, "Index out of range, start index does not leave enough bits for num_bits");
        if num_bits == 0 {
            BigUInt::new()
        } else if start == self.length - 1 && num_bits == self.length {
            self.clone()
        } else {
            let mut block_idx = start / BLOCK_SIZE;
            let start_offset = start % BLOCK_SIZE;
            if num_bits < start_offset + 1 {
                let left_offset = BLOCK_SIZE - start_offset - 1;
                let right_offset = BLOCK_SIZE - num_bits - left_offset;
                let mask = (BLOCK_MASK << right_offset) >> left_offset;
                BigUInt::from_u64((self.bits[block_idx] & mask) >> right_offset)
            } else {
                let mut rest = num_bits;
                let left_offset = BLOCK_SIZE - start_offset - 1;
                let mut res = BigUInt::from_u64(self.bits[block_idx] & (BLOCK_MASK >> left_offset));
                rest -= BLOCK_SIZE - left_offset;
                while rest >= BLOCK_SIZE {
                    block_idx -= 1;
                    res <<= BLOCK_SIZE;
                    res.bits[0] = self.bits[block_idx];
                    rest -= BLOCK_SIZE;
                }
                if rest > 0 {
                    let right_offset = BLOCK_SIZE - rest;
                    res <<= rest;
                    res.bits[0] |= (self.bits[block_idx] & (BLOCK_MASK << right_offset)) >> right_offset;
                }
                res
            }
        }
    }


    /// Set the specified bit in the BigUInt to a value and return the prior value.
    ///
    /// The function will extend the BigUInt to the required size if a bit outside of the current
    /// size is set to 1 (true). A bit outside the current size set to zero will be ignored.
    ///
    /// Please note that setting the leading bit to zero will shorten the BigUInt to the next bit
    /// that is non-zero.
    /// # Arguments
    /// * index - index of the bit to set.
    /// * bit - te value to assigne to the bit, true for 1, false for 0
    ///
    /// # Returns
    /// * Some(true) - the bit was 1 before
    /// * Some(false) - the bit was 0 before
    /// * None - the bit had no value before
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let mut bi = BigUInt::from_u128(0xF0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0);
    /// assert_eq!(bi.set(16,true), Some(false));
    /// assert_eq!(bi.to_hex_string(),"F0F0F0F0F0F0F0F0F0F0F0F0F0F1F0F0");
    /// ```
    pub fn set(&mut self, index: usize, bit: bool) -> Option<bool> {
        let mut empty = false;
        if index >= self.length {
            if bit {
                self.bits.resize(index / BLOCK_SIZE + 1, 0);
                empty = true;
                self.length = index + 1;
            } else {
                return None;
            }
        }

        let bit_offset = index % BLOCK_SIZE;

        let res = if empty {
            None
        } else {
            Some((self.bits[index / BLOCK_SIZE] >> bit_offset) & 1 == 1)
        };
        if bit {
            self.bits[index / BLOCK_SIZE] |= 1 << bit_offset;
        } else {
            self.bits[index / BLOCK_SIZE] &= !(1 << bit_offset);
            if index == self.length - 1 {
                self.trim()
            }
        }
        res
    }

    pub fn shift_left(&self, rhs: usize) -> BigUInt {
        // TODO: redesign: process blocks to new vec instead of copying it ahead
        if self.is_zero() {
            BigUInt::new()
        } else if rhs == 0 {
            self.clone()
        } else {
            let length = self.length + rhs;
            let mut bits = if rhs / BLOCK_SIZE > 0 {
                let mut bits = vec![0; rhs / BLOCK_SIZE];
                bits.extend(self.bits.iter());
                bits
            } else {
                self.bits.clone()
            };

            if length > bits.len() * BLOCK_SIZE {
                bits.push(0);
            }
            debug_assert!(bits.len() * BLOCK_SIZE >= length);
            let l_shift = rhs as usize % BLOCK_SIZE;
            if l_shift > 0 {
                let r_shift = BLOCK_SIZE - l_shift;
                let min = usize::max(bits.len() - self.bits.len(), 1);

                for idx in (min..bits.len()).rev() {
                    bits[idx] <<= l_shift;
                    bits[idx] |= bits[idx - 1] >> r_shift;
                }
                bits[min - 1] <<= l_shift;
            }

            let res = BigUInt {
                length,
                bits,
            };
            #[cfg(feature = "debug_checks")]
                res.check(function!());
            res
        }
    }

    pub fn shift_left_into(&mut self, rhs: usize) {
        // TODO: redesign: process blocks to new vec instead of copying it ahead
        if rhs == 0 || self.is_zero() {} else {
            let new_length = self.length + rhs;
            let old_blocks = self.bits.len();
            if rhs as usize / BLOCK_SIZE > 0 {
                let mut bits = vec![0; rhs / BLOCK_SIZE];
                bits.extend(self.bits.iter());
                self.bits = bits;
            }

            if new_length > self.bits.len() * BLOCK_SIZE {
                self.bits.push(0);
            }

            assert!(self.bits.len() * BLOCK_SIZE >= new_length);
            let shift = rhs as usize % BLOCK_SIZE;
            if shift > 0 {
                let rev_shift = BLOCK_SIZE - shift;
                let min = usize::max(self.bits.len() - old_blocks, 1);

                for idx in (min..self.bits.len()).rev() {
                    self.bits[idx] <<= shift;
                    self.bits[idx] |= self.bits[idx - 1] >> rev_shift;
                }
                self.bits[min - 1] <<= shift;
            }
            self.length = new_length;
            #[cfg(feature = "debug_checks")]
                self.check(function!());

        }
    }

    pub fn shift_right(&self, rhs: usize) -> Self {
        if rhs >= self.length {
            // everything shifted away
            BigUInt::new()
        } else if rhs == 0 {
            // nothing to do
            self.clone()
        } else {
            // remaining right shift after dropping blocks
            let r_shift = rhs % BLOCK_SIZE;
            let bits = if r_shift == 0 {
                // no remaining right shift, just drop blocks
                self.bits[rhs / BLOCK_SIZE..].to_vec()
            } else {
                // some remaining right shift, drop blocks & shift
                let l_shift = BLOCK_SIZE - r_shift;
                // skip all blocks that are shifted out
                let blocks = self.bits[rhs / BLOCK_SIZE..].to_vec();
                debug_assert!(blocks.len() > 0, "no blocks left");
                let mut new_block = blocks[0] >> r_shift;
                let mut bits: Vec<u64> = blocks.iter().skip(1).map(|block| {
                    let res = new_block | (*block << l_shift);
                    new_block = *block >> r_shift;
                    res
                }).collect();
                // push the last one
                bits.push(new_block);
                bits
            };
            let mut res = BigUInt {
                length: self.length - rhs,
                bits,
            };
            res.trim();
            #[cfg(feature = "debug_checks")]
                res.check(function!());
            res
        }
    }

    pub fn shift_right_into(&mut self, rhs: usize) {
        if rhs >= self.length {
            // everything shifted away
            self.length = 0;
            self.bits = Vec::new();
        } else if rhs == 0 {
            // nothing to do
        } else {
            // remaining right shift after dropping blocks
            let r_shift = rhs % BLOCK_SIZE;
            let bits = if r_shift == 0 {
                // no remaining right shift, just drop blocks
                self.bits[rhs / BLOCK_SIZE..].to_vec()
            } else {
                // some remaining right shift, drop blocks & shift
                let l_shift = BLOCK_SIZE - r_shift;
                // skip all blocks that are shifted out
                let blocks = self.bits[rhs / BLOCK_SIZE..].to_vec();
                debug_assert!(blocks.len() > 0, "no blocks left");
                let mut new_block = blocks[0] >> r_shift;
                let mut bits: Vec<u64> = blocks.iter().skip(1).map(|block| {
                    let res = new_block | (*block << l_shift);
                    new_block = *block >> r_shift;
                    res
                }).collect();
                // push the last one
                bits.push(new_block);
                bits
            };
            self.length = self.length - rhs;
            self.bits = bits;
            self.trim();
            #[cfg(feature = "debug_checks")]
                self.check(function!());

        }
    }

    pub fn shift_out(&mut self, num_bits: usize) -> BigUInt {
        // eprintln!("shift_out({},{}): length: {}", self.to_hex_string(), num_bits, self.length);
        assert!(num_bits < self.length, "index out of range");
        if num_bits == self.length {
            let res = (*self).clone();
            self.length = 0;
            self.bits.clear();
            res
        } else {
            let mut last_size = self.length % BLOCK_SIZE;
            if last_size == 0 { last_size = BLOCK_SIZE; }
            match last_size.cmp(&num_bits) {
                Ordering::Greater => {
                    // eprintln!("shift_out({},{}): Ordering::Greater", self.to_hex_string(), num_bits);
                    let last_idx = self.bits.len() - 1;
                    let last = &mut self.bits[last_idx];
                    // eprintln!("  last:\t{:b}", *last);
                    let mask = (BLOCK_MASK >> (BLOCK_SIZE - num_bits)) << (last_size - num_bits);
                    // eprintln!("  mask:\t{:b}", mask);
                    let res = BigUInt::from_u64((*last & mask) >> (last_size - num_bits));
                    // eprintln!("  res:\t{}", res.to_bin_string());
                    *last &= !mask;
                    // eprintln!("  rest:\t{:b}", *last);
                    self.length -= num_bits;
                    self.trim();
                    #[cfg(feature = "debug_checks")]
                        res.check(function!());
                    res
                }
                Ordering::Equal => {
                    // eprintln!("shift_out({},{}): Ordering::Equal", self.to_hex_string(), num_bits);
                    let res = BigUInt::from_u64(*self.bits.last().expect("Unexpected empty BigUInt"));
                    self.length -= num_bits;
                    self.bits.resize(self.bits.len() - 1, 0);
                    #[cfg(feature = "debug_checks")]
                        res.check(function!());
                    res
                }
                Ordering::Less => {
                    // eprintln!("shift_out({},{}): Ordering::Less", self.to_hex_string(), num_bits);
                    let mut rest = num_bits;
                    let mut block_idx = self.bits.len() - 1;
                    let mut res = BigUInt::from_u64(self.bits[block_idx]);
                    // eprintln!("self:\t{}", self.to_bin_string());
                    // eprintln!("res:\t{}", res.to_bin_string());
                    block_idx -= 1;
                    rest -= last_size;
                    // eprintln!("rest:\t{}", rest);
                    while rest >= BLOCK_SIZE {
                        res <<= BLOCK_SIZE;
                        res.bits[0] = self.bits[block_idx];
                        block_idx -= 1;
                        rest -= BLOCK_SIZE;
                        // eprintln!("res:\t{}", res.to_bin_string());
                        // eprintln!("rest:\t{}", rest);
                    }

                    if rest > 0 {
                        let mask = BLOCK_MASK << (BLOCK_SIZE - rest);
                        // eprintln!("mask:\t{:b}", mask);
                        // eprintln!("from:\t{:b}", self.bits[block_idx]);
                        res <<= rest;
                        res.bits[0] |= (self.bits[block_idx] & mask) >> (BLOCK_SIZE - rest);
                        // eprintln!("res:\t{}", res.to_bin_string());
                        self.bits[block_idx] &= !mask;
                    }
                    self.length -= num_bits;
                    self.bits.resize(self.length / BLOCK_SIZE + if self.length % BLOCK_SIZE > 0 { 1 } else { 0 }, 0);
                    self.trim();
                    #[cfg(feature = "debug_checks")]
                        res.check(function!());

                    res
                }
            }
        }
    }

    pub fn iter(&self) -> BitIterator {
        BitIterator {
            bits: self,
            pos: self.length,
        }
    }
}

pub struct BitIterator<'a> {
    bits: &'a BigUInt,
    pos: usize,
}

impl<'a> Iterator for BitIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bits.length == 0 || self.pos == 0 {
            None
        } else {
            self.pos -= 1;
            let bit_offset = self.pos % BLOCK_SIZE;
            Some((self.bits.bits[self.pos / BLOCK_SIZE] >> bit_offset) & 1 == 1)
        }
    }
}
