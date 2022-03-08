use std::fmt::{Debug, Formatter};
use std::cmp::Ordering;
use crate::bit_field::BitField;

mod bit_field;




pub struct BigInt {
    sign: Option<bool>,
    bits: BitField
}

const HEX_DIGITS: [char;16] = ['0','1','2','3','4','5','6','7','8','9','A','B','C','D','E','F'];

impl BigInt {
    pub fn from_u8(from: u8) -> BigInt {
        BigInt {
            sign: None,
            bits: BitField::from_u8_lt(from)
        }
    }

    pub fn from_u16(from: u16) -> BigInt {
        BigInt {
            sign: None,
            bits: BitField::from_u16(from)
        }
    }

    pub fn from_u32(from: u32) -> BigInt {
        BigInt {
            sign: None,
            bits: BitField::from_u32_lt(from)
        }
    }

    pub fn from_u64(from: u64) -> BigInt {
        BigInt {
            sign: None,
            bits: BitField::from_u64_lt(from)
        }
    }

    pub fn zero_s() -> BigInt {
        BigInt {
            sign: Some(false),
            bits: BitField::new()
        }
    }

    pub fn zero_u() -> BigInt {
        BigInt {
            sign: None,
            bits: BitField::new()
        }
    }

    pub fn is_zero(&self) -> bool {
        self.bits.is_empty()
    }
/*
    pub  fn add(&self, other: &BigInt) -> BigInt {
        if other.is_zero() {
            BigInt { bits: self.bits.clone() }
        } else if self.is_zero() {
            BigInt { bits: other.bits.clone() }
        } else {
            let mut temp = Vec::new();
            let (op1, op2) = if self.bits.len() > other.bits.len() {
                for _idx in 0..self.bits.len() - other.bits.len() {
                    temp.push(false);
                }
                temp.append(&mut other.bits.clone());
                (&self.bits, &temp)
            } else if other.bits.len() > self.bits.len() {
                for _idx in 0..other.bits.len() - self.bits.len() {
                    temp.push(false);
                }
                temp.append(&mut self.bits.clone());
                (&other.bits, &temp)
            } else {
                (&self.bits, &other.bits)
            };

            let mut values = Vec::new();
            let mut overflow = false;
            for (bit1, bit2)  in op1.iter().zip(op2).rev() {
                if *bit1 {
                    if *bit2 {
                        if overflow {
                            values.push(true)
                        } else {
                            values.push(false)
                        }
                        overflow = true;
                    } else {
                        if overflow {
                            values.push(false);
                            overflow = true;
                        } else {
                            values.push(true);
                        }
                    }
                }  else {
                    if *bit2 {
                        if overflow {
                            values.push(false);
                            overflow = true;
                        } else {
                            values.push(true);
                        }
                    } else {
                        if overflow {
                            values.push(true);
                            overflow = false;
                        } else {
                            values.push(false);
                        }
                    }
                }
            }
            if overflow {
                values.push(true);
            }
            values.reverse();

            BigInt { bits: values }
        }
    }

    pub  fn subtract(&self, other: &BigInt) -> BigInt {
        if self < other {
            panic!("subtraction wraps around");
        } else {
            if other.is_zero() {
                BigInt { bits: self.bits.clone() }
            } else {
                let mut sub = Vec::new();
                for _idx in 0..self.bits.len() - other.bits.len() {
                    sub.push(false);
                }
                sub.append(&mut other.bits.clone());
                let mut overflow = 0;
                let mut values = Vec::new();
                for (bit1, bit2) in self.bits.iter().zip(sub).rev() {
                    let mut this = if *bit1 { 1 } else { 0 };
                    let other = if bit2 { 1 } else { 0 } + overflow;
                    if other > this {
                        overflow = 1;
                        this += 2;
                    }
                    values.push(if this - other == 1 { true } else { false });
                }
                while let Some(false) = values.last() {
                    values.pop();
                }
                values.reverse();
                BigInt { bits: values }
            }
        }
    }

    pub fn multiply(&self, other: &BigInt) -> BigInt {
        if self.is_zero() || other.is_zero() {
            BigInt::zero()
        } else {
            let mut values = Vec::new();
            let mut pad = Vec::new();
            for bit1 in other.bits.iter().rev() {
                if *bit1 {
                    let mut value = self.bits.clone();
                    value.append( &mut pad.clone());
                    values.push(BigInt {bits: value});
                }
                pad.push(false);
            }

            let mut res = BigInt { bits: Vec::new() };
            for value in values {
                res = res.add(&value)
            }
            res
        }
    }

    pub fn divide(&self, other: &BigInt) -> BigInt {
        if self.is_zero() {
            BigInt::zero()
        } else if other.is_zero() {
            panic!("Division by zero");
        } else if self < other {
            BigInt::zero()
        } else {
            let mut values = Vec::new();
            let mut pad = Vec::new();
            for bit1 in other.bits.iter().rev() {
                if *bit1 {
                    let mut value = self.bits.clone();
                    value.append( &mut pad.clone());
                    values.push(BigInt {bits: value});
                }
                pad.push(false);
            }

            let mut res = BigInt { bits: Vec::new() };
            for value in values {
                res = res.add(&value)
            }
            res
        }
    }

    pub fn to_binary_string(&self) -> String {
        let mut res = String::new();
        if self.is_zero() {
            res.push('0');
        } else {
            for digit in &self.bits {
                if *digit {
                    res.push('1');
                } else {
                    res.push('0');
                }
            }
        }
        res
    }

    pub fn to_hex_string(&self) -> String {
        let mut res = String::new();
        if self.is_zero() {
            res.push('0');
        } else {
            let offset = self.bits.len() - 1;
            eprintln!("to_hex_string: <offset {}", offset);

            let mut values = Vec::new();
            let mut hex_digit = 0;
            for (idx, digit) in self.bits.iter().rev().enumerate() {
                if (idx > 0) && (idx % 4 == 0) {
                    values.push(HEX_DIGITS[hex_digit]);
                    hex_digit = 0;
                }
                if *digit {
                    hex_digit += 1 << idx % 4;
                }
            }

            values.push(HEX_DIGITS[hex_digit]);

            for digit in values.iter().rev() {
                res.push(*digit);
            }
        }
        res
    }

    pub fn to_dec_string(&self) -> String {
        let mut res = String::new();
        let mut values = Vec::new();
        let mut dec_digit = 0;
        for (idx, digit) in self.big_int.iter().rev().enumerate() {

        }

        res

    }
*/
}
/*
impl Debug for BigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{}", self.to_hex_string())
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }

    fn ne(&self, other: &Self) -> bool {
        self.cmp(other) != Ordering::Equal
    }
}

impl Eq for BigInt {
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.bits.len() > other.bits.len() {
            Ordering::Greater
        } else if other.bits.len() > self.bits.len() {
            Ordering::Less
        } else {
            for (bit1,bit2) in self.bits.iter().zip(&other.bits) {
                if *bit1 && !*bit2 {
                    return Ordering::Greater;
                } else if *bit2 && !*bit1 {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use crate::BigInt;
/*
    #[test]
    fn test_from_u32() {
        let test = 0b11010101;
        let big_int = BigInt::from_u32(test);
        assert_eq!(big_int.to_binary_string(),"11010101")
    }

    #[test]
    fn test_from_u64() {
        let test = 0b11010101u64;
        let big_int = BigInt::from_u64(test);
        assert_eq!(big_int.to_binary_string(),"11010101");
        let test = 0xf0f0f0f0f0f0f0f0u64;
        let big_int = BigInt::from_u64(test);
        assert_eq!(big_int.to_binary_string(),"1111000011110000111100001111000011110000111100001111000011110000")

    }

    #[test]
    fn test_to_hex_string() {
        let test = 0x1234ABCD;
        let big_int = BigInt::from_u32(test);
        assert_eq!(big_int.to_hex_string(),"1234ABCD");
    }

    #[test]
    fn test_subtract() {
        let bi1 = BigInt::from_u32(0b1000);
        let bi2 = BigInt::from_u32(0b111);
        assert_eq!(bi1.subtract(&bi2).to_binary_string(), "1");

        let bi1 = BigInt::from_u32(0b1111);
        let bi2 = BigInt::from_u32(0b1110);
        assert_eq!(bi1.subtract(&bi2).to_binary_string(), "1");

        let bi1 = BigInt::from_u32(0b1110);
        let bi2 = BigInt::from_u32(0b1100);
        assert_eq!(bi1.subtract(&bi2).to_binary_string(), "10");

    }

    #[test]
    #[should_panic]
    fn test_subtract_fail() {
        let bi1 = BigInt::from_u32(0b1000);
        let bi2 = BigInt::from_u32(0b111);

        bi2.subtract(&bi1);
    }

    #[test]
    fn test_add() {
        let bi1 = BigInt::from_u32(0b1);
        let bi2 = BigInt::from_u32(0b1);
        assert_eq!(bi1.add(&bi2).to_binary_string(), "10");

        let bi1 = BigInt::from_u32(0b1010);
        let bi2 = BigInt::from_u32(0b110);
        assert_eq!(bi1.add(&bi2).to_binary_string(), "10000");

        assert_eq!(bi2.add(&bi1).to_binary_string(), "10000");

        let bi1 = BigInt::from_u32(0b1010);
        let bi2 = BigInt::from_u32(0b10);
        assert_eq!(bi1.add(&bi2).to_binary_string(), "1100");
    }

    #[test]
    fn test_multiply() {
        let bi1 = BigInt::from_u32(0);
        let bi2 = BigInt::from_u32(0b10);
        assert_eq!(bi1.multiply(&bi2).to_binary_string(), "0");

        let bi1 = BigInt::from_u32(0b10);
        let bi2 = BigInt::from_u32(0b10);
        assert_eq!(bi1.multiply(&bi2).to_binary_string(), "100");

        let bi1 = BigInt::from_u32(0b111);
        let bi2 = BigInt::from_u32(0b101);
        assert_eq!(bi1.multiply(&bi2), BigInt::from_u32(35));
    }
*/
}
