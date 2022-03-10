#![crate_name = "simple_big_int"]

use std::ops::{Shl, ShlAssign, BitAnd, BitAndAssign, BitOrAssign, BitOr, AddAssign, Add, SubAssign, Sub, MulAssign, Mul, DivAssign};
use std::cmp::Ordering;

type Block = u64;

const BLOCK_SIZE: usize = 64;
const BLOCK_MASK: u64 = 0xFFFFFFFFFFFFFFFF;
const BIT_64: u64 = 0x8000000000000000;
const BIT_65: u128 = 0x10000000000000000;

const HEX_DIGITS: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

#[cfg(test)]
mod tests;

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
    pub fn new() -> BigUInt {
        BigUInt {
            length: 0,
            bits: Vec::new(),
        }
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

    /// Retrieve the number of bits stored in the BigUInt
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let bi = BigUInt::from_u32(0xFFFFFFF);
    /// assert_eq!(bi.length(), 28);
    /// ```

    pub fn length(&self) -> usize {
        self.length
    }

    /// Check if the BigUInt is 0 / empty
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    /// let bi = BigUInt::from_u32(0xFFFFFFF);
    /// assert!(!bi.is_empty());
    /// ```

    pub fn is_empty(&self) -> bool {
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
    pub fn iter(&self) -> BitIterator {
        BitIterator {
            bits: self,
            pos: self.length,
        }
    }

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

    /// Subtract from self and store the result in self.
    /// Due to BigUInt not being able to implement the Copy trait and the std::ops::AddAssign trait
    /// consuming the right hand side operator the use of -= can be unefficient, having to clone
    /// the right hand side operator.
    /// This function works around that restriction, it is used by the std::ops::AddAssign implementation
    ///
    /// # Arguments
    /// * other - the value to be subtracted
    ///
    /// # Examples
    /// ```
    /// use simple_big_int::BigUInt;
    ///  let mut bi = BigUInt::from_u128(0xF0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0);
    ///  bi.sub_from(&BigUInt::from_u32(0xF0F0));
    ///  assert_eq!(bi.to_hex_string(),"F0F0F0F0F0F0F0F0F0F0F0F0F0F00000");
    /// ```

    pub fn sub_from(&mut self, other: &BigUInt) {
        // TODO: find out when trim is required, instead of doing it generally
        match (*self).cmp(other) {
            Ordering::Less => panic!("integer underflow"),
            Ordering::Equal => {
                self.length = 0;
                self.bits.clear();
            }
            Ordering::Greater => {
                let mut overflow = false;
                for (block1, block2) in self.bits.iter_mut().zip(other.bits.iter()) {
                    let mut work = *block1 as u128;
                    if overflow {
                        if work > 0 {
                            work -= 1;
                            overflow = false;
                        } else {
                            work = BIT_65 - 1;
                            overflow = true;
                        }
                    }

                    if work < *block2 as u128 {
                        overflow = true;
                        work = work + BIT_65 - *block2 as u128;
                    } else {
                        work -= *block2 as u128;
                    }

                    *block1 = work as u64;
                }
                if overflow {
                    for block in &mut self.bits[other.bits.len()..] {
                        if *block > 0 {
                            *block -= 1;
                            overflow = false;
                            break;
                        } else {
                            *block = BLOCK_MASK;
                        }
                    }
                    assert!(!overflow);
                }
                self.trim();
            }
        }
    }

    pub fn to_bin_string(&self) -> String {
        if self.is_empty() {
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
                    res
                }
                Ordering::Equal => {
                    // eprintln!("shift_out({},{}): Ordering::Equal", self.to_hex_string(), num_bits);
                    let res = BigUInt::from_u64(*self.bits.last().expect("Unexpected empty BigUInt"));
                    self.length -= num_bits;
                    self.bits.resize(self.bits.len() - 1, 0);
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
                    // eprintln!("self:\t{}", self.to_bin_string());
                    res
                }
            }
        }
    }

    pub fn to_hex_string(&self) -> String {
        //eprintln!("to_hex_string: 0b{}", self.to_bin_string());
        if self.is_empty() {
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
}

impl Default for BigUInt {
    fn default() -> Self {
        Self::new()
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

// TODO: create alternatives to all std::ops traits as they are often inefficient due to
//       BigUInt not implementing the copy trait
//       See SubAssign / BigUInt::sub_from()

impl Add for BigUInt {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        // eprintln!("add_assign({},{})", self.to_hex_string(), other.to_hex_string());
        if other.is_empty() {
            self
        } else if self.is_empty() {
            other
        } else {
            let mut overflow = false;
            let mut bits = vec![];
            for (block1, block2) in self.bits.iter().zip(other.bits.iter()) {
                let res = *block1 as u128 + *block2 as u128 + if overflow { 1 } else { 0 };
                bits.push((res & BLOCK_MASK as u128) as Block);
                overflow = res & 0x10000000000000000 == 0x10000000000000000;
            }
            // eprintln!("add_assign({},{}) after first loop, overflow: {}", self.to_hex_string(), other.to_hex_string(), overflow);
            if self.bits.len() > other.bits.len() {
                if overflow {
                    for block in &self.bits[other.bits.len()..] {
                        let res = *block as u128 + 1;
                        bits.push((res & BLOCK_MASK as u128) as Block);
                        overflow = res & 0x10000000000000000 == 0x10000000000000000;
                        if !overflow {
                            break;
                        }
                    }
                }
                if overflow {
                    bits.push(1);
                } else {
                    bits.extend_from_slice(&self.bits[bits.len()..]);
                }
            } else if other.bits.len() > self.bits.len() {
                if overflow {
                    for block in &other.bits[self.bits.len()..] {
                        let res = *block as u128 + 1;
                        bits.push(res as u64);
                        overflow = res & 0x10000000000000000 == 0x10000000000000000;
                        if !overflow {
                            break;
                        }
                    }
                }
                if overflow {
                    bits.push(1);
                } else {
                    bits.extend_from_slice(&other.bits[bits.len()..]);
                }
            } else if overflow {
                bits.push(1);
            }

            let length = if overflow {
                (bits.len() - 1) * BLOCK_SIZE + 1
            } else {
                bits.len() * BLOCK_SIZE
            };
            let mut res = BigUInt {
                length,
                bits,
            };
            if !overflow {
                res.trim();
            }
            res
        }
    }
}

impl AddAssign for BigUInt {
    fn add_assign(&mut self, other: Self) {
        // eprintln!("add_assign({},{})", self.to_hex_string(), other.to_hex_string());
        if self.is_empty() {
            self.length = other.length;
            self.bits = other.bits
        } else if other.is_empty() {} else {
            let mut overflow = false;
            for (block1, block2) in self.bits.iter_mut().zip(other.bits.iter()) {
                let res = *block1 as u128 + *block2 as u128 + if overflow { 1 } else { 0 };
                *block1 = (res & BLOCK_MASK as u128) as Block;
                overflow = res & 0x10000000000000000 == 0x10000000000000000;
            }
            // eprintln!("add_assign({},{}) after first loop, overflow: {}", self.to_hex_string(), other.to_hex_string(), overflow);
            if self.bits.len() > other.bits.len() {
                if overflow {
                    for block in &mut self.bits[other.bits.len()..] {
                        let res = *block as u128 + 1;
                        *block = (res & BLOCK_MASK as u128) as Block;
                        overflow = res & 0x10000000000000000 == 0x10000000000000000;
                        if !overflow {
                            break;
                        }
                    }
                    if overflow {
                        self.bits.push(1);
                    }
                }
            } else if other.bits.len() > self.bits.len() {
                if overflow {
                    for block in &other.bits[self.bits.len()..] {
                        let res = *block as u128 + 1;
                        self.bits.push(res as u64);
                        overflow = res & 0x10000000000000000 == 0x10000000000000000;
                        if !overflow {
                            break;
                        }
                    }
                }
                if overflow {
                    self.bits.push(1);
                } else {
                    self.bits.extend_from_slice(&other.bits[self.bits.len()..]);
                }
            } else if overflow {
                self.bits.push(1);
            }
            if overflow {
                self.length = (self.bits.len() - 1) * BLOCK_SIZE + 1;
            } else {
                self.trim()
            }
        }
    }
}

impl Sub for BigUInt {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match (self).cmp(&other) {
            Ordering::Less => panic!("integer underflow"),
            Ordering::Equal => BigUInt::new(),
            Ordering::Greater => {
                let mut bits = vec![];
                let mut overflow = false;
                for (block1, block2) in self.bits.iter().zip(other.bits.iter()) {
                    let mut work = *block1 as u128;
                    if overflow {
                        if work > 0 {
                            work -= 1;
                            overflow = false;
                        } else {
                            work = BLOCK_MASK as u128;
                            overflow = true;
                        }
                    }

                    if work < *block2 as u128 {
                        overflow = true;
                        work = work + BIT_65 - *block2 as u128;
                    } else {
                        work -= *block2 as u128;
                    }

                    bits.push(work as u64);
                }

                if overflow {
                    for block in &self.bits[other.bits.len()..] {
                        if *block > 0 {
                            bits.push(*block - 1);
                            overflow = false;
                            break;
                        } else {
                            bits.push(BLOCK_MASK);
                        }
                    }
                    assert!(!overflow);
                } else {
                    bits.extend_from_slice(&self.bits[other.bits.len()..]);
                }

                let mut res = BigUInt {
                    length: bits.len() * BLOCK_SIZE,
                    bits,
                };
                res.trim();
                res
            }
        }
    }
}

impl SubAssign for BigUInt {
    fn sub_assign(&mut self, other: Self) {
        self.sub_from(&other);
    }
}

impl Mul for BigUInt {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        if self.is_empty() {
            self
        } else if other.is_empty() {
            other
        } else {
            let mut res_list = vec![];
            for (idx1, block1) in other.bits.iter().enumerate() {
                for (idx2, block2) in self.bits.iter().enumerate() {
                    let mut res = BigUInt::from_u128(*block1 as u128 * *block2 as u128);
                    res <<= (idx1 + idx2) * BLOCK_SIZE;
                    res_list.push(res);
                }
            }
            let mut sum = BigUInt::new();
            for res in res_list {
                sum += res;
            }
            sum
        }
    }
}

impl MulAssign for BigUInt {
    fn mul_assign(&mut self, other: Self) {
        if self.is_empty() {} else if other.is_empty() {
            self.length = 0;
            self.bits.clear();
        } else {
            let mut res_list = vec![];
            for (idx1, block1) in other.bits.iter().enumerate() {
                for (idx2, block2) in self.bits.iter().enumerate() {
                    let mut res = BigUInt::from_u128(*block1 as u128 * *block2 as u128);
                    res <<= (idx1 + idx2) * BLOCK_SIZE;
                    res_list.push(res);
                }
            }
            let mut sum = BigUInt::new();
            for res in res_list {
                sum += res;
            }
            self.length = sum.length;
            self.bits = sum.bits
        }
    }
}

impl DivAssign for BigUInt {
    fn div_assign(&mut self, other: Self) {
        assert!(!other.is_empty(),"Division by zero");
        match (*self).cmp(&other) {
            Ordering::Less => {
                self.length = 0;
                self.bits.clear();
            }
            Ordering::Equal => {
                self.length = 1;
                self.bits.resize(1,0);
                self.bits[0] = 1;
            }
            Ordering::Greater => {
                /*
                if D = 0 then error(DivisionByZeroException) end
                    Q := 0                  -- Initialize quotient and remainder to zero
                    R := 0
                    for i := n − 1 .. 0 do  -- Where n is number of bits in N
                    R := R << 1           -- Left-shift R by 1 bit
                    R(0) := N(i)          -- Set the least-significant bit of R equal to bit i of the numerator
                    if R ≥ D then
                        R := R − D
                        Q(i) := 1
                    end
                end
                */
                let mut res = BigUInt::new();
                let mut modulo = BigUInt::new();
                for idx in (0..self.length).rev() {
                    modulo <<= 1;
                    modulo.set(0,self.get(idx).expect("Unexpected bit index out of range"));
                    if modulo >= other {
                        modulo.sub_from(&other);
                        res.set(idx, true);
                    }
                }
                self.length = res.length;
                self.bits = res.bits;
            }
        }
    }
}

impl ShlAssign<usize> for BigUInt {
    fn shl_assign(&mut self, rhs: usize) {
        if rhs == 0 || self.is_empty() {} else {
            let new_length = self.length + rhs;
            let old_blocks = self.bits.len();
            if rhs / BLOCK_SIZE > 0 {
                let mut bits = vec![0; rhs / BLOCK_SIZE];
                bits.extend(self.bits.iter());
                self.bits = bits;
            }

            if new_length > self.bits.len() * BLOCK_SIZE {
                self.bits.push(0);
            }

            assert!(self.bits.len() * BLOCK_SIZE >= new_length);
            let shift = rhs % BLOCK_SIZE;
            if shift > 0 {
                let rev_shift = BLOCK_SIZE - shift;
                let mask = BLOCK_MASK << rev_shift;


                let min = usize::max(self.bits.len() - old_blocks, 1);

                for idx in (min..self.bits.len()).rev() {
                    self.bits[idx] <<= shift;
                    self.bits[idx] |= (self.bits[idx - 1] & mask) >> rev_shift;
                }
                self.bits[min - 1] <<= shift;
            }
            self.length = new_length;
        }
    }
}

impl Shl<usize> for BigUInt {
    type Output = BigUInt;

    fn shl(self, rhs: usize) -> Self::Output {
        if rhs == 0 || self.is_empty() {
            self
        } else {
            let new_length = self.length + rhs;
            let mut bits = if rhs / BLOCK_SIZE > 0 {
                let mut bits = vec![0; rhs / BLOCK_SIZE];
                bits.extend(self.bits.iter());
                bits
            } else {
                self.bits.clone()
            };

            if new_length > bits.len() * BLOCK_SIZE {
                bits.push(0);
            }
            assert!(bits.len() * BLOCK_SIZE >= new_length);
            let shift = rhs % BLOCK_SIZE;
            if shift > 0 {
                let rev_shift = BLOCK_SIZE - shift;
                let mask = BLOCK_MASK << rev_shift;

                let min = usize::max(bits.len() - self.bits.len(), 1);

                for idx in (min..bits.len()).rev() {
                    bits[idx] <<= shift;
                    bits[idx] |= (bits[idx - 1] & mask) >> rev_shift;
                }
                bits[min - 1] <<= shift;
            }

            BigUInt {
                length: new_length,
                bits,
            }
        }
    }
}

impl BitAnd for BigUInt {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a & b`
    fn bitand(self, rhs: Self) -> Self {
        let mut bits = vec![];
        for (block1, block2) in self.bits.iter().zip(rhs.bits.iter()) {
            bits.push(*block1 & *block2);
        }


        let mut res = BigUInt {
            length: usize::max(self.length, rhs.length),
            bits,
        };
        res.trim();
        res
    }
}

impl BitAndAssign for BigUInt {
    // rhs is the "right-hand side" of the expression `a &= b`
    fn bitand_assign(&mut self, rhs: Self) {
        for (block1, block2) in self.bits.iter_mut().zip(rhs.bits.iter()) {
            *block1 &= *block2;
        }

        self.trim();
    }
}

impl BitOr for BigUInt {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a &= b`
    fn bitor(self, rhs: Self) -> Self {
        let mut bits = vec![];
        for (block1, block2) in self.bits.iter().zip(rhs.bits.iter()) {
            bits.push(*block1 | *block2);
        }

        if self.bits.len() < rhs.bits.len() {
            bits.extend_from_slice(&rhs.bits[self.bits.len()..]);
        }

        let mut res = BigUInt {
            length: usize::max(self.length, rhs.length),
            bits,
        };

        res.trim();
        res
    }
}

impl BitOrAssign for BigUInt {
    // rhs is the "right-hand side" of the expression `a &= b`
    fn bitor_assign(&mut self, rhs: Self) {
        for (block1, block2) in self.bits.iter_mut().zip(rhs.bits.iter()) {
            *block1 |= *block2;
        }

        if self.bits.len() < rhs.bits.len() {
            self.bits.extend_from_slice(&rhs.bits[self.bits.len()..]);
        }
        self.length = usize::max(self.length, rhs.length);

        self.trim();
    }
}

impl Eq for BigUInt {}

impl PartialOrd for BigUInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigUInt {
    fn cmp(&self, other: &Self) -> Ordering {
        // eprintln!("cmp 0x{} length: {}, 0x{} length {}", self.to_hex_string(),self.length(), other.to_hex_string(), other.length());
        match self.length.cmp(&other.length) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                for (block1, block2) in self.bits.iter().zip(other.bits.iter()) {
                    match block1.cmp(block2) {
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => ()
                    }
                }
                Ordering::Equal
            }
        }
    }
}

