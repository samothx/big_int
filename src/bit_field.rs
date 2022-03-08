type BLOCK = u64;
const BLOCK_SIZE: usize = 64;
const HEX_DIGITS: [char;16] = ['0','1','2','3','4','5','6','7','8','9','A','B','C','D','E','F'];


pub struct BitField {
    length: usize,
    bits: Vec<BLOCK>
}

impl BitField {
    pub fn new() -> BitField {
        BitField {
            length: 0,
            bits: Vec::new()
        }
    }

    pub fn from_u8(from: u8) -> BitField {
        if from == 0 {
            BitField::new()
        } else {
            BitField {
                length: 8,
                bits: vec![from as BLOCK]
            }
        }
    }

    pub fn from_u8_sparse(from: u8) -> BitField {
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
                bits: vec![from as BLOCK]
            }
        } else {
            BitField::new()
        }
    }

    pub fn from_u16(from: u16) -> BitField {
        BitField {
            length: 16,
            bits: vec![from as BLOCK]
        }
    }

    pub fn from_u16_sparse(from: u16) -> BitField {
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
                bits: vec![from as BLOCK]
            }
        } else {
            BitField::new()
        }
    }

    pub fn from_u32_sparse(from: u32) -> BitField {
        if from > 0 {
            let mut work = from;

            let mut length = 0usize;
            for idx in (0..=32).rev() {
                if work & 0x80000000 == 0x80000000 {
                    length = idx;
                    break;
                } else {
                    work = work << 1;
                }
            }
            BitField {
                length,
                bits: vec![from as BLOCK]
            }
        } else {
            BitField::new()
        }
    }

    pub fn from_u64_sparse(from: u64) -> BitField {
        let mut work = from;

        let mut length = 0usize;
        for idx in 64..0 {
            if work & 0x8000000000000000 == 0x8000000000000000 {
                length = idx;
                break;
            } else {
                work = work << 1;
            }
        }
        BitField {
            length,
            bits: vec![from as BLOCK]
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn iter(&self) -> BitFieldIterator {
       BitFieldIterator{
           bits: self,
           pos: self.length
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
            for _ in self.length..=index/BLOCK_SIZE {
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

    pub fn to_hex_string(&self) -> String {
        eprintln!("to_hex_string: 0b{}", self.to_bin_string());
        if self.is_empty() {
            eprintln!("to_hex_string: 0b{} - empty", self.to_bin_string());
            String::from('0')
        } else {
            eprintln!("to_hex_string: 0b{} - length: {}", self.to_bin_string(), self.length);
            let offset = self.length % 4;
            let mut res = String::new();
            let mut digit = 0;
            let offset = self.length - 1;
            for (index, bit) in self.iter().enumerate() {
                digit = digit << 1;
                if bit {
                    digit += 1;
                }
                eprintln!("to_hex_string: 0b{} - loop idx: {}, value: {}, digit: {:x}", self.to_bin_string(), index, bit, digit);
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
    pos: usize
}

impl<'a> Iterator for BitFieldIterator<'a> {
    type Item = (bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == 0 {
            None
        } else {
            self.pos -= 1;
            let bit_offset = self.pos % BLOCK_SIZE;
            Some((self.bits.bits[self.pos / BLOCK_SIZE] >> bit_offset) & 1 == 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u8_sparse() {
        let bf = BitField::from_u8_sparse(0);
        assert!(bf.is_empty());
        let bf = BitField::from_u8_sparse(1);
        assert!(!bf.is_empty());
        assert_eq!(bf.length(), 1);
        assert_eq!(bf.to_bin_string(), "1");
        let bf = BitField::from_u8_sparse(0x80);
        assert!(!bf.is_empty());
        assert_eq!(bf.length(), 8);
        assert_eq!(bf.to_bin_string(), "10000000");
        assert_eq!(bf.to_hex_string(), "80");
    }

    #[test]
    fn test_from_u16_sparse() {
        let bf = BitField::from_u16_sparse(0);
        assert!(bf.is_empty());
        let bf = BitField::from_u16_sparse(1);
        assert!(!bf.is_empty());
        assert_eq!(bf.length(), 1);
        assert_eq!(bf.to_bin_string(), "1");
        let bf = BitField::from_u16_sparse(0x8000);
        assert!(!bf.is_empty());
        assert_eq!(bf.length(), 16);
        assert_eq!(bf.to_bin_string(), "1000000000000000");
    }

    #[test]
    fn test_from_u32_sparse() {
        let bf = BitField::from_u32_sparse(0);
        assert!(bf.is_empty());
        let bf = BitField::from_u32_sparse(1);
        assert!(!bf.is_empty());
        assert_eq!(bf.length(), 1);
        assert_eq!(bf.to_bin_string(), "1");
        let bf = BitField::from_u32_sparse(0x80000000);
        assert!(!bf.is_empty());
        assert_eq!(bf.length(), 32);
        assert_eq!(bf.to_hex_string(), "80000000");
    }

    #[test]
    fn test_to_hex_string() {
        let bf = BitField::from_u16_sparse(0b10000000);
        assert_eq!(bf.to_hex_string(), "80");
        let bf = BitField::from_u16_sparse(0b1000000);
        assert_eq!(bf.to_hex_string(), "40");
        let bf = BitField::from_u16_sparse(0b100000);
        assert_eq!(bf.to_hex_string(), "20");
        let bf = BitField::from_u16_sparse(0b10000);
        assert_eq!(bf.to_hex_string(), "10");
        let bf = BitField::from_u16_sparse(0b1000);
        assert_eq!(bf.to_hex_string(), "8");
        let bf = BitField::from_u32_sparse(0x39CE739C);
        assert_eq!(bf.to_hex_string(), "39CE739C");
    }

}
