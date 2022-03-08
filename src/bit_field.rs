use std::ops::{Shl, ShlAssign, BitAnd, BitAndAssign, BitOrAssign};

type BLOCK = u64;

const BLOCK_SIZE: usize = 64;
const BLOCK_MASK: u64 = 0xFFFFFFFFFFFFFFFF;
const HIGH_BIT: u64 = 0x8000000000000000;

const HEX_DIGITS: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];


#[derive(Clone)]
pub struct BitField {
    length: usize,
    bits: Vec<BLOCK>,
}

impl BitField {
    pub fn new() -> BitField {
        BitField {
            length: 0,
            bits: Vec::new(),
        }
    }

    pub fn from_u8(from: u8) -> BitField {
        if from == 0 {
            BitField::new()
        } else {
            BitField {
                length: 8,
                bits: vec![from as BLOCK],
            }
        }
    }


    pub fn from_u8_lt(from: u8) -> BitField {
        if from > 0 {
            let mut work = from;

            let mut length = 0usize;
            for idx in (1..=8).rev() {
                if work & 0x80 == 0x80 {
                    length = idx;
                    break;
                } else {
                    work = work << 1;
                }
            }
            BitField {
                length,
                bits: vec![from as BLOCK],
            }
        } else {
            BitField::new()
        }
    }

    pub fn from_u16(from: u16) -> BitField {
        BitField {
            length: 16,
            bits: vec![from as BLOCK],
        }
    }

    pub fn from_u16_lt(from: u16) -> BitField {
        if from > 0 {
            let mut work = from;

            let mut length = 0usize;
            for idx in (1..=16).rev() {
                if work & 0x8000 == 0x8000 {
                    length = idx;
                    break;
                } else {
                    work = work << 1;
                }
            }
            BitField {
                length,
                bits: vec![from as BLOCK],
            }
        } else {
            BitField::new()
        }
    }

    pub fn from_u32_lt(from: u32) -> BitField {
        if from > 0 {
            let mut work = from;

            let mut length = 0usize;
            for idx in (1..=32).rev() {
                if work & 0x80000000 == 0x80000000 {
                    length = idx;
                    break;
                } else {
                    work = work << 1;
                }
            }
            BitField {
                length,
                bits: vec![from as BLOCK],
            }
        } else {
            BitField::new()
        }
    }

    pub fn from_u64_lt(from: u64) -> BitField {
        if from > 0 {
            let mut work = from;

            let mut length = 0usize;
            for idx in (1..=64).rev() {
                if work & 0x8000000000000000 == 0x8000000000000000 {
                    length = idx;
                    break;
                } else {
                    work = work << 1;
                }
            }
            BitField {
                length,
                bits: vec![from as BLOCK],
            }
        } else {
            BitField::new()
        }
    }

    pub fn from_u128_lt(from: u128) -> BitField {
        if from > 0 {
            let (mut bits, mut high_block) = if from > BLOCK_MASK as u128 {
                let mut bits = vec![];
                bits.push((from & BLOCK_MASK as u128) as u64);
                let high_block = (from >> 64) as u64;
                bits.push(high_block);
                (bits, high_block)
            } else {
                let mut bits = vec![];
                let high_block = from as u64;
                bits.push(high_block);
                (bits, high_block)
            };

            let mut length = 0usize;
            for idx in (1..=BLOCK_SIZE).rev() {
                if high_block & HIGH_BIT == HIGH_BIT {
                    length = idx;
                    break;
                } else {
                    high_block = high_block << 1;
                }
            }
            if bits.len() > 1 {
                length += BLOCK_SIZE
            }
            BitField {
                length,
                bits,
            }
        } else {
            BitField::new()
        }
    }


    pub fn length(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn iter(&self) -> BitFieldIterator {
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
            for _ in self.length..=index / BLOCK_SIZE {
                self.bits.push(0);
            }
            empty = true;
        }

        let bit_offset = index % BLOCK_SIZE;

        let res = if empty {
            None
        } else {
            Some((self.bits[index / BLOCK_SIZE] >> bit_offset) & 1 == 1)
        };
        if bit {
            self.bits[index / BLOCK_SIZE] = self.bits[index / BLOCK_SIZE] | (1 << bit_offset);
        } else {
            self.bits[index / BLOCK_SIZE] = self.bits[index / BLOCK_SIZE] & !(1 << bit_offset);
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

    pub fn trim_left(&mut self) {
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
                self.length = length;
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
            let offset = self.length % 4;
            let mut res = String::new();
            let mut digit = 0;
            let offset = self.length - 1;
            for (index, bit) in self.iter().enumerate() {
                digit = digit << 1;
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

struct BitFieldIterator<'a> {
    bits: &'a BitField,
    pos: usize,
}

impl<'a> Iterator for BitFieldIterator<'a> {
    type Item = (bool);

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

impl ShlAssign<usize> for BitField {
    fn shl_assign(&mut self, rhs: usize) {
        if rhs == 0 {
            ()
        } else if self.is_empty() {
            self.length = rhs;
            self.bits = vec![0; rhs / BLOCK_SIZE];
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

impl Shl<usize> for BitField {
    type Output = BitField;

    fn shl(self, rhs: usize) -> Self::Output {
        if rhs == 0 {
            self.clone()
        } else if self.is_empty() {
            BitField {
                length: rhs,
                bits: vec![0; rhs / BLOCK_SIZE],
            }
        } else {
            let new_length = self.length + rhs;
            let mut bits = if rhs / BLOCK_SIZE > 0 {
                let mut bits = vec![0; rhs / BLOCK_SIZE];
                bits.extend(self.bits.iter());
                bits
            } else {
                let bits = self.bits.clone();
                bits
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

            BitField {
                length: new_length,
                bits,
            }
        }
    }
}

impl BitAnd for BitField {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a & b`
    fn bitand(self, rhs: Self) -> Self {
        let mut bits = vec![];
        for (block1, block2) in self.bits.iter().zip(rhs.bits.iter()) {
            bits.push(*block1 & *block2);
        }

        if self.bits.len() > rhs.bits.len() {
            for block in &self.bits[rhs.bits.len()..] {
                bits.push(*block);
            }
        } else if self.bits.len() < rhs.bits.len() {
            for block in &rhs.bits[self.bits.len()..] {
                bits.push(*block);
            }
        }
        BitField {
            length: usize::max(self.length, rhs.length),
            bits
        }
    }
}

impl BitAndAssign for BitField {
    // rhs is the "right-hand side" of the expression `a &= b`
    fn bitand_assign(&mut self, rhs: Self) {
        for (block1, block2) in self.bits.iter_mut().zip(rhs.bits.iter()) {
            *block1 &= *block2;
        }

        if self.bits.len() < rhs.bits.len() {
            self.bits.extend(vec![0;rhs.bits.len() - self.bits.len()]);
        }
        self.length = usize::max(self.length, rhs.length);
    }
}

impl BitOrAssign for BitField {
    // rhs is the "right-hand side" of the expression `a &= b`
    fn bitor_assign(&mut self, rhs: Self) {
        for (block1, block2) in self.bits.iter_mut().zip(rhs.bits.iter()) {
            *block1 |= *block2;
        }

        if self.bits.len() < rhs.bits.len() {
            self.bits.extend_from_slice( &rhs.bits[self.bits.len()..]);
        }
        self.length = usize::max(self.length, rhs.length);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u8_sparse() {
        let bf = BitField::from_u8_lt(0);
        assert!(bf.is_empty());
        let bf = BitField::from_u8_lt(1);
        assert!(!bf.is_empty());
        assert_eq!(bf.length(), 1);
        assert_eq!(bf.to_bin_string(), "1");
        let bf = BitField::from_u8_lt(0x80);
        assert!(!bf.is_empty());
        assert_eq!(bf.length(), 8);
        assert_eq!(bf.to_bin_string(), "10000000");
        assert_eq!(bf.to_hex_string(), "80");
    }

    #[test]
    fn test_from_u16_sparse() {
        let bf = BitField::from_u16_lt(0);
        assert!(bf.is_empty());
        let bf = BitField::from_u16_lt(1);
        assert!(!bf.is_empty());
        assert_eq!(bf.length(), 1);
        assert_eq!(bf.to_bin_string(), "1");
        let bf = BitField::from_u16_lt(0x8000);
        assert!(!bf.is_empty());
        assert_eq!(bf.length(), 16);
        assert_eq!(bf.to_bin_string(), "1000000000000000");
    }

    #[test]
    fn test_from_u32_sparse() {
        let bf = BitField::from_u32_lt(0);
        assert!(bf.is_empty());
        let bf = BitField::from_u32_lt(1);
        assert!(!bf.is_empty());
        assert_eq!(bf.length(), 1);
        assert_eq!(bf.to_bin_string(), "1");
        let bf = BitField::from_u32_lt(0x80000000);
        assert!(!bf.is_empty());
        assert_eq!(bf.length(), 32);
        assert_eq!(bf.to_hex_string(), "80000000");
    }

    #[test]
    fn test_from_u64_sparse() {
        let bf = BitField::from_u64_lt(0);
        assert!(bf.is_empty());
        let bf = BitField::from_u64_lt(1);
        assert!(!bf.is_empty());
        assert_eq!(bf.length(), 1);
        assert_eq!(bf.to_bin_string(), "1");
        let bf = BitField::from_u64_lt(0x8000000000000000);
        assert!(!bf.is_empty());
        assert_eq!(bf.length(), 64);
        assert_eq!(bf.to_hex_string(), "8000000000000000");
    }

    #[test]
    fn test_from_u128_sparse() {
        let bf = BitField::from_u128_lt(0);
        assert!(bf.is_empty());
        let bf = BitField::from_u128_lt(1);
        assert!(!bf.is_empty());
        assert_eq!(bf.length(), 1);
        assert_eq!(bf.to_bin_string(), "1");
        let bf = BitField::from_u128_lt(0x80000000000000000000000000000000);
        assert!(!bf.is_empty());
        assert_eq!(bf.length(), 128);
        assert_eq!(bf.to_hex_string(), "80000000000000000000000000000000");
    }

    #[test]
    fn test_to_hex_string() {
        let bf = BitField::from_u16_lt(0b10000000);
        assert_eq!(bf.to_hex_string(), "80");
        let bf = BitField::from_u16_lt(0b1000000);
        assert_eq!(bf.to_hex_string(), "40");
        let bf = BitField::from_u16_lt(0b100000);
        assert_eq!(bf.to_hex_string(), "20");
        let bf = BitField::from_u16_lt(0b10000);
        assert_eq!(bf.to_hex_string(), "10");
        let bf = BitField::from_u16_lt(0b1000);
        assert_eq!(bf.to_hex_string(), "8");
        let bf = BitField::from_u32_lt(0x39CE739C);
        assert_eq!(bf.to_hex_string(), "39CE739C");
    }

    #[test]
    fn test_shift() {
        let mut bf = BitField::from_u64_lt(0x1);
        let res = bf << 1;
        assert_eq!(res.to_hex_string(), "2");
        let bf = BitField::from_u64_lt(0x39CE739C);
        let res = bf << 1;
        assert_eq!(res.to_hex_string(), "739CE738");
        let bf = BitField::from_u64_lt(0x739CE739CE739CE7);
        let res = bf << 1;
        assert_eq!(res.to_hex_string(), "E739CE739CE739CE");

        let bf = BitField::from_u64_lt(0x739CE739CE739CE7);
        let res = bf << 2;
        assert_eq!(res.to_bin_string(), format!("{:b}", 0x739CE739CE739CE7u128 << 2));

        let bf = BitField::from_u64_lt(0x739CE739CE739CE7);
        let res = bf << 64;
        assert_eq!(res.to_bin_string(), format!("{:b}", 0x739CE739CE739CE7u128 << 64));
    }

    #[test]
    fn test_shift_assign() {
        let mut bf = BitField::from_u64_lt(0x1);
        bf <<= 1;
        assert_eq!(bf.to_hex_string(), "2");
        let mut bf = BitField::from_u64_lt(0x39CE739C);
        bf <<= 1;
        assert_eq!(bf.to_hex_string(), "739CE738");
        let mut bf = BitField::from_u64_lt(0x739CE739CE739CE7);
        bf <<= 1;
        assert_eq!(bf.to_hex_string(), "E739CE739CE739CE");

        let mut bf = BitField::from_u64_lt(0x739CE739CE739CE7);
        bf <<= 2;
        assert_eq!(bf.to_bin_string(), format!("{:b}", 0x739CE739CE739CE7u128 << 2));

        let mut bf = BitField::from_u64_lt(0x739CE739CE739CE7);
        bf <<= 64;
        assert_eq!(bf.to_bin_string(), format!("{:b}", 0x739CE739CE739CE7u128 << 64));
    }
}
