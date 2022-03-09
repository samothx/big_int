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

#[derive(Clone, PartialEq)]
pub struct BigUInt {
    length: usize,
    bits: Vec<Block>,
}

impl BigUInt {
    pub fn new() -> BigUInt {
        BigUInt {
            length: 0,
            bits: Vec::new(),
        }
    }

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

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn iter(&self) -> BitFieldIterator {
        BitFieldIterator {
            bits: self,
            pos: self.length,
        }
    }

    pub fn get(&self, index: usize) -> Option<bool> {
        if index >= self.length {
            None
        } else {
            let bit_offset = index % BLOCK_SIZE;
            Some((self.bits[index / BLOCK_SIZE] >> bit_offset) & 1 == 1)
        }
    }

    pub fn set(&mut self, index: usize, bit: bool) -> Option<bool> {
        let mut empty = false;
        if index >= self.length {
            if bit {
                for _ in self.length..=index / BLOCK_SIZE {
                    self.bits.push(0);
                }
                empty = true;
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
        }
        res
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
            if last_size == 0 {last_size = BLOCK_SIZE; }
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
                    res
                },
                Ordering::Equal => {
                    // eprintln!("shift_out({},{}): Ordering::Equal", self.to_hex_string(), num_bits);
                    let res = BigUInt::from_u64(*self.bits.last().expect("Unexpected empty BigUInt"));
                    self.length -= num_bits;
                    self.bits.resize(self.bits.len() - 1, 0);
                    res
                },
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

struct BitFieldIterator<'a> {
    bits: &'a BigUInt,
    pos: usize,
}

impl<'a> Iterator for BitFieldIterator<'a> {
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

                let mut res = BigUInt{
                    length: bits.len() * BLOCK_SIZE,
                    bits
                };
                res.trim();
                res
            }
        }
    }
}

impl SubAssign for BigUInt {
    fn sub_assign(&mut self, other: Self) {
        match (*self).cmp(&other) {
            Ordering::Less => panic!("integer underflow"),
            Ordering::Equal => {
                self.length = 0;
                self.bits.clear();
            },
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
            }
        }
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
                for (idx2,block2) in self.bits.iter().enumerate() {
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
        if self.is_empty() {
        } else if other.is_empty() {
            self.length = 0;
            self.bits.clear();
        } else {
            let mut res_list = vec![];
            for (idx1, block1) in other.bits.iter().enumerate() {
                for (idx2,block2) in self.bits.iter().enumerate() {
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
        match (*self).cmp(&other) {
            Ordering::Less => {
                self.length = 0;
                self.bits.clear();
            },
            Ordering::Equal => {
                self.length = 1;
                self.bits.clear();
                self.bits.push(1);
            },
            Ordering::Greater => {
                todo!()
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

