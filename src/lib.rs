use std::ops::{Shl, ShlAssign, BitAnd, BitAndAssign, BitOrAssign, BitOr, AddAssign};

type Block = u64;

const BLOCK_SIZE: usize = 64;
const BLOCK_MASK: u64 = 0xFFFFFFFFFFFFFFFF;
const HIGH_BIT: u64 = 0x8000000000000000;

const HEX_DIGITS: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

#[cfg(test)]
mod tests;

#[derive(Clone)]
pub struct BigInt {
    length: usize,
    bits: Vec<Block>,
}

impl BigInt {
    pub fn new() -> BigInt {
        BigInt {
            length: 0,
            bits: Vec::new(),
        }
    }

    pub fn from_u8(from: u8) -> BigInt {
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
            BigInt {
                length,
                bits: vec![from as Block],
            }
        } else {
            BigInt::new()
        }
    }

    pub fn from_u16(from: u16) -> BigInt {
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
            BigInt {
                length,
                bits: vec![from as Block],
            }
        } else {
            BigInt::new()
        }
    }

    pub fn from_u32(from: u32) -> BigInt {
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
            BigInt {
                length,
                bits: vec![from as Block],
            }
        } else {
            BigInt::new()
        }
    }

    pub fn from_u64(from: u64) -> BigInt {
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
            BigInt {
                length,
                bits: vec![from as Block],
            }
        } else {
            BigInt::new()
        }
    }

    pub fn from_u128(from: u128) -> BigInt {
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
                if high_block & HIGH_BIT == HIGH_BIT {
                    length = idx;
                    break;
                } else {
                    high_block <<= 1;
                }
            }
            if bits.len() > 1 {
                length += BLOCK_SIZE
            }
            BigInt {
                length,
                bits,
            }
        } else {
            BigInt::new()
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
            self.bits[index / BLOCK_SIZE] &=  !(1 << bit_offset);
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

    fn make_sparse(&mut self) {
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

impl Default for BigInt {
    fn default() -> Self {
        Self::new()
    }
}

struct BitFieldIterator<'a> {
    bits: &'a BigInt,
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

impl AddAssign for BigInt {
    fn add_assign(&mut self, other: Self) {
        // eprintln!("add_assign({},{})", self.to_hex_string(), other.to_hex_string());
        if other.is_empty() {} else if self.is_empty() {
            self.length = other.length;
            self.bits = other.bits;
        } else {
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
                        self.length = (self.bits.len() - 1) * BLOCK_SIZE + 1;
                    }
                }
                self.make_sparse()
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
                    if overflow {
                        self.bits.push(1);
                    }
                }
                if other.bits.len() > self.bits.len() {
                    for block in &other.bits[self.bits.len()..] {
                        self.bits.push(*block);
                    }
                }
                self.make_sparse()
            } else if overflow {
                self.bits.push(1);
                self.length = (self.bits.len() - 1) * BLOCK_SIZE + 1;
            } else {
                self.make_sparse();
            }
        }
    }
}

impl ShlAssign<usize> for BigInt {
    fn shl_assign(&mut self, rhs: usize) {
        if rhs == 0 || self.is_empty() {
        } else {
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

impl Shl<usize> for BigInt {
    type Output = BigInt;

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

            BigInt {
                length: new_length,
                bits,
            }
        }
    }
}

impl BitAnd for BigInt {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a & b`
    fn bitand(self, rhs: Self) -> Self {
        let mut bits = vec![];
        for (block1, block2) in self.bits.iter().zip(rhs.bits.iter()) {
            bits.push(*block1 & *block2);
        }


        let mut res = BigInt {
            length: usize::max(self.length, rhs.length),
            bits,
        };
        res.make_sparse();
        res
    }
}

impl BitAndAssign for BigInt {
    // rhs is the "right-hand side" of the expression `a &= b`
    fn bitand_assign(&mut self, rhs: Self) {
        for (block1, block2) in self.bits.iter_mut().zip(rhs.bits.iter()) {
            *block1 &= *block2;
        }

        self.make_sparse();
    }
}

impl BitOr for BigInt {
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

        let mut res = BigInt {
            length: usize::max(self.length, rhs.length),
            bits,
        };

        res.make_sparse();
        res
    }
}

impl BitOrAssign for BigInt {
    // rhs is the "right-hand side" of the expression `a &= b`
    fn bitor_assign(&mut self, rhs: Self) {
        for (block1, block2) in self.bits.iter_mut().zip(rhs.bits.iter()) {
            *block1 |= *block2;
        }

        if self.bits.len() < rhs.bits.len() {
            self.bits.extend_from_slice(&rhs.bits[self.bits.len()..]);
        }
        self.length = usize::max(self.length, rhs.length);

        self.make_sparse();
    }
}


